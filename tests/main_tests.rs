use anyhow::Result;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use tempfile::TempDir;
use xor::db::FileRecord;

/// 创建临时测试文件
fn create_test_file(dir: &Path, name: &str, content: &[u8]) -> Result<PathBuf> {
    let path = dir.join(name);
    let mut file = File::create(&path)?;
    file.write_all(content)?;
    Ok(path)
}

#[test]
fn test_file_creation() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let test_file = create_test_file(temp_dir.path(), "test.txt", b"Hello, World!")?;

    // 验证文件存在
    assert!(test_file.exists());

    // 验证文件内容
    let content = fs::read_to_string(&test_file)?;
    assert_eq!(content, "Hello, World!");

    Ok(())
}

#[test]
fn test_file_metadata() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let test_file = create_test_file(temp_dir.path(), "test.txt", b"Hello")?;

    // 验证文件大小
    let metadata = fs::metadata(&test_file)?;
    assert_eq!(metadata.len(), 5);

    // 验证文件修改时间存在
    let _modified = metadata.modified()?;

    Ok(())
}

#[test]
fn test_empty_file() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let test_file = create_test_file(temp_dir.path(), "empty.txt", b"")?;

    // 验证文件存在
    assert!(test_file.exists());

    // 验证文件大小为 0
    let metadata = fs::metadata(&test_file)?;
    assert_eq!(metadata.len(), 0);

    Ok(())
}

#[test]
fn test_compression_ratio_calculation() {
    // 测试压缩率计算
    let record = FileRecord {
        id: None,
        relative_path: "test.txt".to_string(),
        modified_time: "2025-12-10 10:00:00".to_string(),
        original_hash: "hash".to_string(),
        output_hash: "hash".to_string(),
        original_size: 1000,
        output_size: 500,
        created_at: "2025-12-10 10:00:00".to_string(),
    };

    // 压缩率应该是 50%
    let ratio = (record.output_size as f64 / record.original_size as f64) * 100.0;
    assert_eq!(ratio, 50.0);
}

#[test]
fn test_zero_size_compression_ratio() {
    let record = FileRecord {
        id: None,
        relative_path: "empty.txt".to_string(),
        modified_time: "2025-12-10 10:00:00".to_string(),
        original_hash: "hash".to_string(),
        output_hash: "hash".to_string(),
        original_size: 0,
        output_size: 0,
        created_at: "2025-12-10 10:00:00".to_string(),
    };

    // 原始大小为 0 时应该特殊处理
    if record.original_size > 0 {
        let _ratio = (record.output_size as f64 / record.original_size as f64) * 100.0;
    } else {
        // 应该显示 "N/A" 或类似的标识
        assert_eq!(record.original_size, 0);
    }
}

#[test]
fn test_high_compression_ratio() {
    // 测试高压缩率场景
    let record = FileRecord {
        id: None,
        relative_path: "compressed.txt".to_string(),
        modified_time: "2025-12-10 10:00:00".to_string(),
        original_hash: "hash".to_string(),
        output_hash: "hash".to_string(),
        original_size: 10000,
        output_size: 500,
        created_at: "2025-12-10 10:00:00".to_string(),
    };

    // 压缩率应该是 5%
    let ratio = (record.output_size as f64 / record.original_size as f64) * 100.0;
    assert_eq!(ratio, 5.0);
}

#[test]
fn test_large_file_sizes() {
    // 测试大文件大小计算
    let record = FileRecord {
        id: None,
        relative_path: "large.bin".to_string(),
        modified_time: "2025-12-10 10:00:00".to_string(),
        original_hash: "hash".to_string(),
        output_hash: "hash".to_string(),
        original_size: 1073741824, // 1 GB
        output_size: 536870912,    // 512 MB
        created_at: "2025-12-10 10:00:00".to_string(),
    };

    // 验证大小值
    assert_eq!(record.original_size, 1024 * 1024 * 1024);
    assert_eq!(record.output_size, 512 * 1024 * 1024);

    // 压缩率应该是 50%
    let ratio = (record.output_size as f64 / record.original_size as f64) * 100.0;
    assert_eq!(ratio, 50.0);
}
