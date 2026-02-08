use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub id: Option<i64>,
    pub timestamp: DateTime<Utc>,
    pub mode: String, // "short", "medium", "long"
    pub wpm: f64,
    pub raw_wpm: f64,
    pub accuracy: f64,
    pub consistency: f64,
    pub quote_length: i64,
    pub duration_seconds: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct UserStats {
    pub total_tests: i64,
    pub best_wpm: f64,
    pub avg_wpm: f64,
    pub avg_accuracy: f64,
    pub total_time_seconds: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default = "default_theme")]
    pub theme: String,

    #[serde(default = "default_mode")]
    pub default_mode: String,

    #[serde(default = "default_time")]
    pub default_time: u64,
}

fn default_theme() -> String {
    "catppuccin-mocha".to_string()
}
fn default_mode() -> String {
    "medium".to_string()
}
fn default_time() -> u64 {
    60
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: default_theme(),
            default_mode: default_mode(),
            default_time: default_time(),
        }
    }
}
