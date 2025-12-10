use aes_gcm::{
    Aes256Gcm, Key, Nonce,
    aead::{Aead, KeyInit},
};
use anyhow::{Context, Result};
use csv::Writer;
use pbkdf2::pbkdf2_hmac;
use rand::RngCore;
use rayon::prelude::*;
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};
use walkdir::WalkDir;
use zstd::stream::Encoder;

mod db;
use db::{Database, FileRecord, LogRecord};

const MAGIC: &[u8; 4] = b"ZENC";
const VERSION: u8 = 1;
const PBKDF2_ITERS: u32 = 100_000;
const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 12;
const ZSTD_WORKERS: u32 = 4; // Zstd 内部线程数

fn main() -> Result<()> {
    let input_dir = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "./input".to_string());
    let output_dir = std::env::args()
        .nth(2)
        .unwrap_or_else(|| "./output".to_string());
    let password = std::env::args()
        .nth(3)
        .unwrap_or_else(|| "default_password".to_string());

    println!("📁 输入目录: {}", input_dir);
    println!("📁 输出目录: {}", output_dir);
    println!("🔐 密码已设置");
    println!("💾 数据库位置: {}", Database::get_db_path_string()?);
    println!("🚀 使用 Rayon 多线程 + Zstd 多线程压缩 + SIMD 加速哈希\n");

    let input_path = Path::new(&input_dir);
    let output_path = Path::new(&output_dir);

    // 创建输出目录
    fs::create_dir_all(output_path)?;

    // 初始化数据库
    let db = Arc::new(Mutex::new(Database::new()?));

    // 收集所有文件路径
    let file_paths: Vec<PathBuf> = WalkDir::new(input_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_path_buf())
        .collect();

    let total_files = file_paths.len();
    println!("📊 找到 {} 个文件\n", total_files);

    let password = Arc::new(password);
    let input_path = Arc::new(input_path.to_path_buf());
    let output_path = Arc::new(output_path.to_path_buf());

    // 用于批量收集需要写入数据库的记录和日志
    let pending_records = Arc::new(Mutex::new(Vec::new()));
    let pending_logs = Arc::new(Mutex::new(Vec::new()));

    // 使用 Rayon 并行处理文件
    let results: Vec<(FileRecord, String)> = file_paths
        .par_iter()
        .filter_map(|file_path| {
            match process_file_with_check(
                file_path,
                &input_path,
                &output_path,
                &password,
                &db,
                &pending_records,
                &pending_logs,
            ) {
                Ok(Some((record, status))) => {
                    println!("{} {}", status, record.relative_path);
                    Some((record, status))
                }
                Ok(None) => None,
                Err(e) => {
                    let error_msg = format!("❌ 错误处理 {:?}: {}", file_path, e);
                    eprintln!("{}", error_msg);

                    // 记录错误日志到批量队列
                    if let Ok(relative_path) = file_path.strip_prefix(&*input_path) {
                        let log = LogRecord {
                            file_path: relative_path.to_string_lossy().to_string(),
                            action: "process".to_string(),
                            status: "error".to_string(),
                            message: e.to_string(),
                            timestamp: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                        };
                        pending_logs.lock().unwrap().push(log);
                    }

                    None
                }
            }
        })
        .collect();

    // 批量写入数据库
    println!("\n💾 正在批量写入数据库...");
    let records_to_write = pending_records.lock().unwrap();
    let logs_to_write = pending_logs.lock().unwrap();

    if !records_to_write.is_empty() {
        db.lock().unwrap().batch_upsert_files(&records_to_write)?;
        println!("✅ 已写入 {} 条文件记录", records_to_write.len());
    }

    if !logs_to_write.is_empty() {
        db.lock().unwrap().batch_add_logs(&logs_to_write)?;
        println!("✅ 已写入 {} 条日志记录", logs_to_write.len());
    }

    // 生成 CSV 清单（兼容性保留）
    let records: Vec<FileRecord> = results.iter().map(|(r, _)| r.clone()).collect();
    let manifest_path = output_path.join("manifest.csv");
    write_manifest(&manifest_path, &records)?;

    // 计算统计信息
    let total_original_size: u64 = records.iter().map(|r| r.original_size).sum();
    let total_output_size: u64 = records.iter().map(|r| r.output_size).sum();
    let compression_ratio = if total_original_size > 0 {
        (total_output_size as f64 / total_original_size as f64) * 100.0
    } else {
        0.0
    };

    println!("\n📋 清单已生成: {}", manifest_path.display());
    println!("🎉 所有文件处理完成！共 {} 个文件", records.len());
    println!("📊 统计信息:");
    println!(
        "   原始总大小: {} ({} MB)",
        format_size(total_original_size),
        total_original_size / 1024 / 1024
    );
    println!(
        "   输出总大小: {} ({} MB)",
        format_size(total_output_size),
        total_output_size / 1024 / 1024
    );
    println!("   压缩率: {:.2}%", compression_ratio);
    println!(
        "   节省空间: {} ({} MB)",
        format_size(total_original_size.saturating_sub(total_output_size)),
        total_original_size.saturating_sub(total_output_size) / 1024 / 1024
    );

    Ok(())
}

/// 格式化文件大小
fn format_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if size >= GB {
        format!("{:.2} GB", size as f64 / GB as f64)
    } else if size >= MB {
        format!("{:.2} MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.2} KB", size as f64 / KB as f64)
    } else {
        format!("{} B", size)
    }
}

/// 检查并处理文件（增量处理逻辑）
fn process_file_with_check(
    file_path: &Path,
    input_path: &Path,
    output_path: &Path,
    password: &str,
    db: &Arc<Mutex<Database>>,
    pending_records: &Arc<Mutex<Vec<FileRecord>>>,
    pending_logs: &Arc<Mutex<Vec<LogRecord>>>,
) -> Result<Option<(FileRecord, String)>> {
    let relative_path = file_path
        .strip_prefix(input_path)?
        .to_str()
        .context("路径转换失败")?
        .to_string();

    // 获取当前文件的修改时间
    let current_modified_time = get_modified_time(file_path)?;

    // 检查数据库中是否存在该文件
    let existing_record = db.lock().unwrap().file_exists(&relative_path)?;

    let should_process = if let Some(existing) = &existing_record {
        // 文件存在于数据库中，检查是否需要更新
        if existing.modified_time != current_modified_time {
            // 修改时间不同，进一步检查 hash
            let current_hash = compute_file_hash_simd(file_path)?;

            if existing.original_hash != current_hash {
                // Hash 不同，需要重新处理
                queue_log(
                    pending_logs,
                    &relative_path,
                    "check",
                    "changed",
                    &format!("文件已变化 (修改时间和哈希均不同)"),
                );
                true
            } else {
                // Hash 相同但修改时间不同（可能只是 touch 了文件）
                queue_log(
                    pending_logs,
                    &relative_path,
                    "check",
                    "skip",
                    &format!("文件未实际变化 (仅修改时间变化)"),
                );
                false
            }
        } else {
            // 修改时间相同，跳过处理
            false
        }
    } else {
        // 数据库中不存在，需要处理
        queue_log(pending_logs, &relative_path, "check", "new", "新文件");
        true
    };

    if !should_process {
        return Ok(None);
    }

    // 执行实际的处理
    match process_file(file_path, input_path, output_path, password) {
        Ok(record) => {
            // 添加到批量写入队列
            pending_records.lock().unwrap().push(record.clone());

            // 记录成功日志到队列
            queue_log(
                pending_logs,
                &relative_path,
                "process",
                "success",
                "文件处理成功",
            );

            let status = if existing_record.is_some() {
                "🔄 更新:"
            } else {
                "✅ 新增:"
            };

            Ok(Some((record, status.to_string())))
        }
        Err(e) => {
            // 记录失败日志到队列
            queue_log(
                pending_logs,
                &relative_path,
                "process",
                "failed",
                &e.to_string(),
            );
            Err(e)
        }
    }
}

/// 将日志添加到队列（用于批量写入）
fn queue_log(
    pending_logs: &Arc<Mutex<Vec<LogRecord>>>,
    file_path: &str,
    action: &str,
    status: &str,
    message: &str,
) {
    let log = LogRecord {
        file_path: file_path.to_string(),
        action: action.to_string(),
        status: status.to_string(),
        message: message.to_string(),
        timestamp: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    };
    pending_logs.lock().unwrap().push(log);
}

/// 记录日志到数据库（立即写入，用于旧代码兼容）
#[allow(dead_code)]
fn log_action(
    db: &Arc<Mutex<Database>>,
    file_path: &str,
    action: &str,
    status: &str,
    message: &str,
) -> Result<()> {
    let log = LogRecord {
        file_path: file_path.to_string(),
        action: action.to_string(),
        status: status.to_string(),
        message: message.to_string(),
        timestamp: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    };
    db.lock().unwrap().add_log(&log)?;
    Ok(())
}
fn process_file(
    file_path: &Path,
    input_path: &Path,
    output_path: &Path,
    password: &str,
) -> Result<FileRecord> {
    let relative_path = file_path
        .strip_prefix(input_path)?
        .to_str()
        .context("路径转换失败")?
        .to_string();

    // 获取原始文件大小
    let original_size = fs::metadata(file_path)?.len();

    // 并行计算原始文件 hash（使用 SIMD 加速）
    let original_hash = compute_file_hash_simd(file_path)?;

    // 获取修改时间
    let modified_time = get_modified_time(file_path)?;

    // 压缩 + 加密
    let output_file_path = output_path.join(&relative_path).with_extension("zstd.enc");

    // 确保输出文件的父目录存在
    if let Some(parent) = output_file_path.parent() {
        fs::create_dir_all(parent)?;
    }

    compress_and_encrypt_mt(file_path, &output_file_path, password)?;

    // 获取输出文件大小
    let output_size = fs::metadata(&output_file_path)?.len();

    // 计算输出文件 hash
    let output_hash = compute_file_hash_simd(&output_file_path)?;

    Ok(FileRecord {
        id: None,
        relative_path,
        modified_time,
        original_hash,
        output_hash,
        original_size,
        output_size,
        created_at: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    })
}

/// 多线程压缩并加密文件
fn compress_and_encrypt_mt(input: &Path, output: &Path, password: &str) -> Result<()> {
    // 1. 读取原始文件
    let mut input_file = File::open(input)?;
    let mut original_data = Vec::new();
    input_file.read_to_end(&mut original_data)?;

    // 2. Zstd 多线程压缩
    let mut encoder = Encoder::new(Vec::new(), 3)?;

    // 启用 Zstd 多线程压缩（需要 zstdmt feature）
    encoder.multithread(ZSTD_WORKERS)?;

    encoder.write_all(&original_data)?;
    let compressed = encoder.finish()?;

    // 3. 生成随机 salt 和 nonce
    let mut salt = [0u8; SALT_LEN];
    let mut nonce_bytes = [0u8; NONCE_LEN];
    rand::thread_rng().fill_bytes(&mut salt);
    rand::thread_rng().fill_bytes(&mut nonce_bytes);

    // 4. 从密码派生密钥（PBKDF2-HMAC-SHA256）
    let mut key_bytes = [0u8; 32];
    pbkdf2_hmac::<Sha256>(password.as_bytes(), &salt, PBKDF2_ITERS, &mut key_bytes);

    // 5. AES-256-GCM 加密
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, compressed.as_ref())
        .map_err(|e| anyhow::anyhow!("加密失败: {:?}", e))?;

    // 6. 写入自定义容器格式
    let mut output_file = File::create(output)?;
    output_file.write_all(MAGIC)?;
    output_file.write_all(&[VERSION])?;
    output_file.write_all(&[SALT_LEN as u8])?;
    output_file.write_all(&salt)?;
    output_file.write_all(&[NONCE_LEN as u8])?;
    output_file.write_all(&nonce_bytes)?;
    output_file.write_all(&ciphertext)?;

    Ok(())
}

/// 使用 SIMD 加速计算文件 SHA256 哈希
/// sha2 crate 会自动使用 CPU 的硬件加速（SHA-NI 指令集）
fn compute_file_hash_simd(path: &Path) -> Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();

    // 使用更大的缓冲区提高吞吐量
    let mut buffer = vec![0u8; 64 * 1024]; // 64KB buffer

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        // sha2 会自动使用 SIMD 指令
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}

/// 获取文件修改时间
fn get_modified_time(path: &Path) -> Result<String> {
    let metadata = fs::metadata(path)?;
    let modified = metadata.modified()?;
    let datetime: chrono::DateTime<chrono::Local> = modified.into();
    Ok(datetime.format("%Y-%m-%d %H:%M:%S").to_string())
}

/// 写入 CSV 清单（线程安全）
fn write_manifest(path: &Path, records: &[FileRecord]) -> Result<()> {
    let mut writer = Writer::from_path(path)?;

    writer.write_record(&[
        "文件路径",
        "最后修改时间",
        "原始文件哈希",
        "输出文件哈希",
        "原始大小(字节)",
        "输出大小(字节)",
        "压缩率",
    ])?;

    for record in records {
        let compression_ratio = if record.original_size > 0 {
            format!(
                "{:.2}%",
                (record.output_size as f64 / record.original_size as f64) * 100.0
            )
        } else {
            "N/A".to_string()
        };

        writer.write_record(&[
            &record.relative_path,
            &record.modified_time,
            &record.original_hash,
            &record.output_hash,
            &record.original_size.to_string(),
            &record.output_size.to_string(),
            &compression_ratio,
        ])?;
    }

    writer.flush()?;
    Ok(())
}
