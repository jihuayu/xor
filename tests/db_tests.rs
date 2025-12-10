use anyhow::Result;
use rusqlite::Connection;
use tempfile::TempDir;
use xor::db::{Database, FileRecord, LogRecord};

/// 创建临时测试数据库
/// 返回数据库和 TempDir，以保持临时目录在测试期间有效
fn create_test_db() -> Result<(Database, TempDir)> {
    let temp_dir = TempDir::new()?;
    let db_path = temp_dir.path().join("test.db");

    let conn = Connection::open(&db_path)?;
    let db = Database { conn };
    db.init_tables()?;

    Ok((db, temp_dir))
}

#[test]
fn test_database_creation() -> Result<()> {
    let (_db, _temp_dir) = create_test_db()?;

    // 验证数据库路径可以获取
    let path = Database::get_db_path_string()?;
    assert!(!path.is_empty());

    Ok(())
}

#[test]
fn test_file_record_upsert() -> Result<()> {
    let (db, _temp_dir) = create_test_db()?;

    let record = FileRecord {
        id: None,
        relative_path: "test/file.txt".to_string(),
        modified_time: "2025-12-10 10:00:00".to_string(),
        original_hash: "abc123".to_string(),
        output_hash: "def456".to_string(),
        original_size: 1024,
        output_size: 512,
        created_at: "2025-12-10 10:00:00".to_string(),
    };

    // 插入记录
    db.upsert_file(&record)?;

    // 验证记录存在
    let found = db.file_exists("test/file.txt")?;
    assert!(found.is_some());

    let found_record = found.unwrap();
    assert_eq!(found_record.relative_path, "test/file.txt");
    assert_eq!(found_record.original_hash, "abc123");
    assert_eq!(found_record.original_size, 1024);

    Ok(())
}

#[test]
fn test_file_record_update() -> Result<()> {
    let (db, _temp_dir) = create_test_db()?;

    // 插入初始记录
    let record1 = FileRecord {
        id: None,
        relative_path: "test/file.txt".to_string(),
        modified_time: "2025-12-10 10:00:00".to_string(),
        original_hash: "abc123".to_string(),
        output_hash: "def456".to_string(),
        original_size: 1024,
        output_size: 512,
        created_at: "2025-12-10 10:00:00".to_string(),
    };
    db.upsert_file(&record1)?;

    // 更新记录
    let record2 = FileRecord {
        id: None,
        relative_path: "test/file.txt".to_string(),
        modified_time: "2025-12-10 11:00:00".to_string(),
        original_hash: "xyz789".to_string(),
        output_hash: "uvw012".to_string(),
        original_size: 2048,
        output_size: 1024,
        created_at: "2025-12-10 11:00:00".to_string(),
    };
    db.upsert_file(&record2)?;

    // 验证更新成功
    let found = db.file_exists("test/file.txt")?;
    assert!(found.is_some());

    let found_record = found.unwrap();
    assert_eq!(found_record.original_hash, "xyz789");
    assert_eq!(found_record.original_size, 2048);

    Ok(())
}

#[test]
fn test_batch_upsert_files() -> Result<()> {
    let (mut db, _temp_dir) = create_test_db()?;

    let records = vec![
        FileRecord {
            id: None,
            relative_path: "file1.txt".to_string(),
            modified_time: "2025-12-10 10:00:00".to_string(),
            original_hash: "hash1".to_string(),
            output_hash: "out1".to_string(),
            original_size: 100,
            output_size: 50,
            created_at: "2025-12-10 10:00:00".to_string(),
        },
        FileRecord {
            id: None,
            relative_path: "file2.txt".to_string(),
            modified_time: "2025-12-10 10:00:00".to_string(),
            original_hash: "hash2".to_string(),
            output_hash: "out2".to_string(),
            original_size: 200,
            output_size: 100,
            created_at: "2025-12-10 10:00:00".to_string(),
        },
        FileRecord {
            id: None,
            relative_path: "file3.txt".to_string(),
            modified_time: "2025-12-10 10:00:00".to_string(),
            original_hash: "hash3".to_string(),
            output_hash: "out3".to_string(),
            original_size: 300,
            output_size: 150,
            created_at: "2025-12-10 10:00:00".to_string(),
        },
    ];

    // 批量插入
    db.batch_upsert_files(&records)?;

    // 验证所有记录
    for record in &records {
        let found = db.file_exists(&record.relative_path)?;
        assert!(found.is_some());
        assert_eq!(found.unwrap().original_hash, record.original_hash);
    }

    Ok(())
}

#[test]
fn test_log_operations() -> Result<()> {
    let (db, _temp_dir) = create_test_db()?;

    let log = LogRecord {
        file_path: "test/file.txt".to_string(),
        action: "process".to_string(),
        status: "success".to_string(),
        message: "File processed successfully".to_string(),
        timestamp: "2025-12-10 10:00:00".to_string(),
    };

    // 添加日志
    db.add_log(&log)?;

    // 获取最近日志
    let logs = db.get_recent_logs(10)?;
    assert!(!logs.is_empty());
    assert_eq!(logs[0].file_path, "test/file.txt");
    assert_eq!(logs[0].status, "success");

    Ok(())
}

#[test]
fn test_batch_add_logs() -> Result<()> {
    let (mut db, _temp_dir) = create_test_db()?;

    let logs = vec![
        LogRecord {
            file_path: "file1.txt".to_string(),
            action: "check".to_string(),
            status: "new".to_string(),
            message: "New file".to_string(),
            timestamp: "2025-12-10 10:00:00".to_string(),
        },
        LogRecord {
            file_path: "file2.txt".to_string(),
            action: "process".to_string(),
            status: "success".to_string(),
            message: "Processed".to_string(),
            timestamp: "2025-12-10 10:01:00".to_string(),
        },
        LogRecord {
            file_path: "file3.txt".to_string(),
            action: "process".to_string(),
            status: "failed".to_string(),
            message: "Error occurred".to_string(),
            timestamp: "2025-12-10 10:02:00".to_string(),
        },
    ];

    // 批量添加日志
    db.batch_add_logs(&logs)?;

    // 获取最近日志
    let recent_logs = db.get_recent_logs(10)?;
    assert_eq!(recent_logs.len(), 3);

    Ok(())
}

#[test]
fn test_get_all_files() -> Result<()> {
    let (mut db, _temp_dir) = create_test_db()?;

    // 插入多个文件记录
    let records = vec![
        FileRecord {
            id: None,
            relative_path: "a.txt".to_string(),
            modified_time: "2025-12-10 10:00:00".to_string(),
            original_hash: "hash_a".to_string(),
            output_hash: "out_a".to_string(),
            original_size: 100,
            output_size: 50,
            created_at: "2025-12-10 10:00:00".to_string(),
        },
        FileRecord {
            id: None,
            relative_path: "b.txt".to_string(),
            modified_time: "2025-12-10 10:00:00".to_string(),
            original_hash: "hash_b".to_string(),
            output_hash: "out_b".to_string(),
            original_size: 200,
            output_size: 100,
            created_at: "2025-12-10 10:00:00".to_string(),
        },
    ];

    db.batch_upsert_files(&records)?;

    // 获取所有文件
    let all_files = db.get_all_files()?;
    assert_eq!(all_files.len(), 2);

    // 验证按路径排序
    assert_eq!(all_files[0].relative_path, "a.txt");
    assert_eq!(all_files[1].relative_path, "b.txt");

    Ok(())
}

#[test]
fn test_file_not_exists() -> Result<()> {
    let (db, _temp_dir) = create_test_db()?;

    // 查询不存在的文件
    let found = db.file_exists("nonexistent.txt")?;
    assert!(found.is_none());

    Ok(())
}

#[test]
fn test_empty_batch_operations() -> Result<()> {
    let (mut db, _temp_dir) = create_test_db()?;

    // 空批量操作应该成功
    db.batch_upsert_files(&[])?;
    db.batch_add_logs(&[])?;

    Ok(())
}
