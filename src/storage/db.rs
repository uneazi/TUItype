use rusqlite::{Connection, Result, params};
use chrono::Utc;
use crate::models::{TestResult, UserStats};

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn open(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        let db = Self { conn };
        db.init_tables()?;
        Ok(db)
    }

    fn init_tables(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS test_results (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp TEXT NOT NULL,
                mode TEXT NOT NULL,
                wpm REAL NOT NULL,
                raw_wpm REAL NOT NULL,
                accuracy REAL NOT NULL,
                consistency REAL NOT NULL,
                quote_length INTEGER NOT NULL,
                duration_seconds INTEGER NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub fn save_result(&self, result: &TestResult) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO test_results 
             (timestamp, mode, wpm, raw_wpm, accuracy, consistency, quote_length, duration_seconds)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                result.timestamp.to_rfc3339(),
                result.mode,
                result.wpm,
                result.raw_wpm,
                result.accuracy,
                result.consistency,
                result.quote_length,
                result.duration_seconds,
            ],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn get_recent_results(&self, limit: usize) -> Result<Vec<TestResult>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, timestamp, mode, wpm, raw_wpm, accuracy, consistency,
                    quote_length, duration_seconds
             FROM test_results
             ORDER BY timestamp DESC
             LIMIT ?1"
        )?;

        let results = stmt.query_map([limit as i64], |row| {
            Ok(TestResult {
                id: Some(row.get(0)?),
                timestamp: row.get::<_, String>(1)?.parse().unwrap_or(Utc::now()),
                mode: row.get(2)?,
                wpm: row.get(3)?,
                raw_wpm: row.get(4)?,
                accuracy: row.get(5)?,
                consistency: row.get(6)?,
                quote_length: row.get(7)?,
                duration_seconds: row.get(8)?,
            })
        })?
        .collect::<Result<Vec<_>>>()?;

        Ok(results)
    }

    pub fn get_stats(&self) -> Result<UserStats> {
        let total_tests: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM test_results",
            [],
            |row| row.get(0),
        )?;

        let best_wpm: f64 = self.conn.query_row(
            "SELECT COALESCE(MAX(wpm), 0.0) FROM test_results",
            [],
            |row| row.get(0),
        )?;

        let avg_wpm: f64 = self.conn.query_row(
            "SELECT COALESCE(AVG(wpm), 0.0) FROM test_results",
            [],
            |row| row.get(0),
        )?;

        let avg_accuracy: f64 = self.conn.query_row(
            "SELECT COALESCE(AVG(accuracy), 0.0) FROM test_results",
            [],
            |row| row.get(0),
        )?;

        let total_time: i64 = self.conn.query_row(
            "SELECT COALESCE(SUM(duration_seconds), 0) FROM test_results",
            [],
            |row| row.get(0),
        )?;

        Ok(UserStats {
            total_tests,
            best_wpm,
            avg_wpm,
            avg_accuracy,
            total_time_seconds: total_time,
        })
    }
}
