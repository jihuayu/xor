use anyhow::{Context, Result};
use rusqlite::{Connection, params};
use std::path::PathBuf;

/// 文件记录
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FileRecord {
    pub id: Option<i64>,
    pub relative_path: String,
    pub modified_time: String,
    pub original_hash: String,
    pub output_hash: String,
    pub original_size: u64,
    pub output_size: u64,
    pub created_at: String,
}

/// 日志记录
#[derive(Debug, Clone)]
pub struct LogRecord {
    pub file_path: String,
    pub action: String,
    pub status: String,
    pub message: String,
    pub timestamp: String,
}

/// 数据库管理器
pub struct Database {
    conn: Connection,
}

impl Database {
    /// 创建或打开数据库
    pub fn new() -> Result<Self> {
        let db_path = Self::get_db_path()?;

        // 确保目录存在
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let conn =
            Connection::open(&db_path).context(format!("无法打开数据库: {}", db_path.display()))?;

        let db = Database { conn };
        db.init_tables()?;

        Ok(db)
    }

    /// 获取数据库路径
    fn get_db_path() -> Result<PathBuf> {
        let home_dir = dirs::home_dir().context("无法获取用户主目录")?;
        Ok(home_dir.join(".xor").join("data.db"))
    }

    /// 初始化数据库表
    fn init_tables(&self) -> Result<()> {
        // 创建文件表
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS files (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                relative_path TEXT NOT NULL UNIQUE,
                modified_time TEXT NOT NULL,
                original_hash TEXT NOT NULL,
                output_hash TEXT NOT NULL,
                original_size INTEGER NOT NULL DEFAULT 0,
                output_size INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;

        // 添加文件大小列（如果表已存在但没有这些列）
        let _ = self.conn.execute(
            "ALTER TABLE files ADD COLUMN original_size INTEGER NOT NULL DEFAULT 0",
            [],
        );
        let _ = self.conn.execute(
            "ALTER TABLE files ADD COLUMN output_size INTEGER NOT NULL DEFAULT 0",
            [],
        );

        // 创建日志表
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS logs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                file_path TEXT NOT NULL,
                action TEXT NOT NULL,
                status TEXT NOT NULL,
                message TEXT,
                timestamp TEXT NOT NULL
            )",
            [],
        )?;

        // 创建索引
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_files_path ON files(relative_path)",
            [],
        )?;

        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_logs_timestamp ON logs(timestamp)",
            [],
        )?;

        Ok(())
    }

    /// 检查文件是否存在于数据库中
    pub fn file_exists(&self, relative_path: &str) -> Result<Option<FileRecord>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, relative_path, modified_time, original_hash, output_hash, 
                    COALESCE(original_size, 0), COALESCE(output_size, 0), created_at
             FROM files WHERE relative_path = ?1",
        )?;

        let mut rows = stmt.query(params![relative_path])?;

        if let Some(row) = rows.next()? {
            Ok(Some(FileRecord {
                id: Some(row.get(0)?),
                relative_path: row.get(1)?,
                modified_time: row.get(2)?,
                original_hash: row.get(3)?,
                output_hash: row.get(4)?,
                original_size: row.get(5)?,
                output_size: row.get(6)?,
                created_at: row.get(7)?,
            }))
        } else {
            Ok(None)
        }
    }

    /// 插入或更新文件记录
    #[allow(dead_code)]
    pub fn upsert_file(&self, record: &FileRecord) -> Result<()> {
        let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        self.conn.execute(
            "INSERT INTO files (relative_path, modified_time, original_hash, output_hash, original_size, output_size, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
             ON CONFLICT(relative_path) DO UPDATE SET
                modified_time = excluded.modified_time,
                original_hash = excluded.original_hash,
                output_hash = excluded.output_hash,
                original_size = excluded.original_size,
                output_size = excluded.output_size,
                updated_at = excluded.updated_at",
            params![
                &record.relative_path,
                &record.modified_time,
                &record.original_hash,
                &record.output_hash,
                &record.original_size,
                &record.output_size,
                &now,
                &now,
            ],
        )?;

        Ok(())
    }

    /// 批量插入或更新文件记录（使用事务）
    pub fn batch_upsert_files(&mut self, records: &[FileRecord]) -> Result<()> {
        if records.is_empty() {
            return Ok(());
        }

        let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let tx = self.conn.transaction()?;

        {
            let mut stmt = tx.prepare(
                "INSERT INTO files (relative_path, modified_time, original_hash, output_hash, original_size, output_size, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
                 ON CONFLICT(relative_path) DO UPDATE SET
                    modified_time = excluded.modified_time,
                    original_hash = excluded.original_hash,
                    output_hash = excluded.output_hash,
                    original_size = excluded.original_size,
                    output_size = excluded.output_size,
                    updated_at = excluded.updated_at"
            )?;

            for record in records {
                stmt.execute(params![
                    &record.relative_path,
                    &record.modified_time,
                    &record.original_hash,
                    &record.output_hash,
                    &record.original_size,
                    &record.output_size,
                    &now,
                    &now,
                ])?;
            }
        }

        tx.commit()?;
        Ok(())
    }

    /// 添加日志记录
    pub fn add_log(&self, log: &LogRecord) -> Result<()> {
        self.conn.execute(
            "INSERT INTO logs (file_path, action, status, message, timestamp)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                &log.file_path,
                &log.action,
                &log.status,
                &log.message,
                &log.timestamp,
            ],
        )?;

        Ok(())
    }

    /// 批量添加日志记录（使用事务）
    pub fn batch_add_logs(&mut self, logs: &[LogRecord]) -> Result<()> {
        if logs.is_empty() {
            return Ok(());
        }

        let tx = self.conn.transaction()?;

        {
            let mut stmt = tx.prepare(
                "INSERT INTO logs (file_path, action, status, message, timestamp)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
            )?;

            for log in logs {
                stmt.execute(params![
                    &log.file_path,
                    &log.action,
                    &log.status,
                    &log.message,
                    &log.timestamp,
                ])?;
            }
        }

        tx.commit()?;
        Ok(())
    }

    /// 获取所有文件记录
    #[allow(dead_code)]
    pub fn get_all_files(&self) -> Result<Vec<FileRecord>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, relative_path, modified_time, original_hash, output_hash, 
                    COALESCE(original_size, 0), COALESCE(output_size, 0), created_at
             FROM files ORDER BY relative_path",
        )?;

        let rows = stmt.query_map([], |row| {
            Ok(FileRecord {
                id: Some(row.get(0)?),
                relative_path: row.get(1)?,
                modified_time: row.get(2)?,
                original_hash: row.get(3)?,
                output_hash: row.get(4)?,
                original_size: row.get(5)?,
                output_size: row.get(6)?,
                created_at: row.get(7)?,
            })
        })?;

        let mut records = Vec::new();
        for record in rows {
            records.push(record?);
        }

        Ok(records)
    }

    /// 获取最近的日志
    #[allow(dead_code)]
    pub fn get_recent_logs(&self, limit: usize) -> Result<Vec<LogRecord>> {
        let mut stmt = self.conn.prepare(
            "SELECT file_path, action, status, message, timestamp 
             FROM logs ORDER BY timestamp DESC LIMIT ?1",
        )?;

        let rows = stmt.query_map(params![limit], |row| {
            Ok(LogRecord {
                file_path: row.get(0)?,
                action: row.get(1)?,
                status: row.get(2)?,
                message: row.get(3)?,
                timestamp: row.get(4)?,
            })
        })?;

        let mut logs = Vec::new();
        for log in rows {
            logs.push(log?);
        }

        Ok(logs)
    }

    /// 获取数据库路径（用于显示）
    pub fn get_db_path_string() -> Result<String> {
        Ok(Self::get_db_path()?.display().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    /// 创建临时测试数据库
    fn create_test_db() -> Result<Database> {
        let temp_dir = TempDir::new()?;
        let db_path = temp_dir.path().join("test.db");

        let conn = Connection::open(&db_path)?;
        let db = Database { conn };
        db.init_tables()?;

        Ok(db)
    }

    #[test]
    fn test_database_creation() -> Result<()> {
        let _db = create_test_db()?;

        // 验证数据库路径可以获取
        let path = Database::get_db_path_string()?;
        assert!(!path.is_empty());

        Ok(())
    }

    #[test]
    fn test_file_record_upsert() -> Result<()> {
        let db = create_test_db()?;

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
        let db = create_test_db()?;

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
        let mut db = create_test_db()?;

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
        let db = create_test_db()?;

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
        let mut db = create_test_db()?;

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
        let mut db = create_test_db()?;

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
        let db = create_test_db()?;

        // 查询不存在的文件
        let found = db.file_exists("nonexistent.txt")?;
        assert!(found.is_none());

        Ok(())
    }

    #[test]
    fn test_empty_batch_operations() -> Result<()> {
        let mut db = create_test_db()?;

        // 空批量操作应该成功
        db.batch_upsert_files(&[])?;
        db.batch_add_logs(&[])?;

        Ok(())
    }
}
