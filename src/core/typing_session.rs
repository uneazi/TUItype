use std::time::{Duration, Instant};

use crate::core::metrics;
use crate::models::TestResult;
use chrono::Utc;

#[derive(Debug, Clone)]
pub struct TypingSession {
    quote: String,
    typed: String,
    started_at: Option<Instant>,
    mistakes: usize,
    is_complete: bool,
    completed_at: Option<Instant>,
    wpm_history: Vec<(Instant, f64)>,
    final_wpm: f64,
    final_accuracy: f64,
    final_duration: Duration,
}

impl TypingSession {
    pub fn new(quote: String) -> Self {
        Self {
            quote,
            typed: String::new(),
            started_at: None,
            mistakes: 0,
            is_complete: false,
            completed_at: None,
            wpm_history: Vec::new(),
            final_wpm: 0.0,
            final_accuracy: 100.0,
            final_duration: Duration::from_secs(0),
        }
    }

    pub fn start(&mut self) {
        if self.started_at.is_none() {
            self.started_at = Some(Instant::now());
        }
    }

    pub fn type_char(&mut self, c: char) -> bool {
        if self.is_complete {
            return false;
        }

        self.start();

        let expected = self.quote.chars().nth(self.typed.len());
        if expected != Some(c) {
            self.mistakes += 1;
        }

        self.typed.push(c);

        // Check for completion
        if self.typed.len() == self.quote.len() {
            let last_typed = self.typed.chars().last();
            let last_quote = self.quote.chars().last();

            if last_typed == last_quote {
                self.complete();
                return true;
            }
        }

        false
    }

    pub fn backspace(&mut self) {
        if !self.is_complete {
            self.typed.pop();
        }
    }

    pub fn delete_word(&mut self) {
        if self.is_complete {
            return;
        }

        // Find the start of the current word (from right)
        let mut start = self.typed.len();

        // Move left until we hit a non-word character or beginning
        while start > 0 {
            let ch = self.typed.as_bytes()[start - 1];
            if ch.is_ascii_whitespace() || !ch.is_ascii_alphanumeric() {
                break;
            }
            start -= 1;
        }

        // Remove characters from start to end
        self.typed.drain(start..);
    }

    fn complete(&mut self) {
        self.is_complete = true;
        self.completed_at = Some(Instant::now());

        let correct = metrics::count_correct_chars(&self.typed, &self.quote);
        self.final_accuracy = metrics::calculate_accuracy(correct, self.typed.len());

        if let Some(start) = self.started_at {
            self.final_duration = start.elapsed();
            self.final_wpm =
                metrics::calculate_wpm(self.typed.len(), self.final_duration.as_secs_f64());
        }
    }

    pub fn update_metrics(&mut self) {
        if self.is_complete {
            return;
        }

        if let Some(start) = self.started_at {
            let elapsed = start.elapsed().as_secs_f64();
            let wpm = metrics::calculate_wpm(self.typed.len(), elapsed);

            if wpm > 0.0 {
                self.wpm_history.push((Instant::now(), wpm));
            }
        }
    }

    pub fn reset(&mut self, new_quote: String) {
        self.quote = new_quote;
        self.typed.clear();
        self.started_at = None;
        self.mistakes = 0;
        self.is_complete = false;
        self.completed_at = None;
        self.wpm_history.clear();
        self.final_wpm = 0.0;
        self.final_accuracy = 100.0;
        self.final_duration = Duration::from_secs(0);
    }

    pub fn restart(&mut self) {
        self.typed.clear();
        self.started_at = None;
        self.mistakes = 0;
        self.is_complete = false;
        self.completed_at = None;
        self.wpm_history.clear();
        self.final_wpm = 0.0;
        self.final_accuracy = 100.0;
        self.final_duration = Duration::from_secs(0);
    }

    // Getters
    pub fn quote(&self) -> &str {
        &self.quote
    }

    pub fn typed(&self) -> &str {
        &self.typed
    }

    pub fn is_complete(&self) -> bool {
        self.is_complete
    }

    pub fn mistakes(&self) -> usize {
        self.mistakes
    }

    pub fn wpm(&self) -> f64 {
        if self.is_complete {
            self.final_wpm
        } else if let Some(start) = self.started_at {
            metrics::calculate_wpm(self.typed.len(), start.elapsed().as_secs_f64())
        } else {
            0.0
        }
    }

    pub fn raw_wpm(&self) -> f64 {
        if let Some(start) = self.started_at {
            metrics::calculate_raw_wpm(self.typed.len(), start.elapsed().as_secs_f64())
        } else {
            0.0
        }
    }

    pub fn accuracy(&self) -> f64 {
        if self.is_complete {
            self.final_accuracy
        } else {
            let correct = metrics::count_correct_chars(&self.typed, &self.quote);
            metrics::calculate_accuracy(correct, self.typed.len().max(1))
        }
    }

    pub fn consistency(&self) -> f64 {
        metrics::calculate_consistency(&self.wpm_history)
    }

    pub fn duration(&self) -> Duration {
        if self.is_complete {
            self.final_duration
        } else if let Some(start) = self.started_at {
            start.elapsed()
        } else {
            Duration::from_secs(0)
        }
    }

    pub fn final_result(&self) -> Option<TestResult> {
        if !self.is_complete {
            return None;
        }

        Some(TestResult {
            id: None,
            timestamp: Utc::now(),
            mode: "medium".to_string(),
            wpm: self.final_wpm,
            raw_wpm: self.raw_wpm(),
            accuracy: self.final_accuracy,
            consistency: self.consistency(),
            quote_length: self.quote.len() as i64,
            duration_seconds: self.final_duration.as_secs() as i64,
        })
    }
}
