use serde::Deserialize;
use rand::prelude::*;
use anyhow::Result;

#[derive(Debug, Clone, Deserialize)]
pub struct Quote {
    pub text: String,
    pub source: String,
    pub length: usize,
    #[allow(dead_code)]
    pub id: usize,
}

// MonkeyType's actual JSON structure
#[derive(Debug, Deserialize)]
struct MonkeyTypeFile {
    quotes: Vec<Quote>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum QuoteMode {
    Short,
    Medium,
    Long,
}

impl QuoteMode {
    pub fn length_range(&self) -> (usize, usize) {
        match self {
            QuoteMode::Short => (0, 100),
            QuoteMode::Medium => (101, 300),  // Match MonkeyType's groups
            QuoteMode::Long => (301, usize::MAX),
        }
    }
}

const QUOTES_JSON: &str = include_str!("../data/english.json");

pub struct QuoteManager {
    quotes: Vec<Quote>,
}

impl QuoteManager {
    pub fn new() -> Result<Self> {
        let file: MonkeyTypeFile = serde_json::from_str(QUOTES_JSON)?;
        Ok(Self { quotes: file.quotes })
    }

    pub fn get_random_quote(&self, mode: QuoteMode) -> Option<&Quote> {
        let (min, max) = mode.length_range();

        let filtered: Vec<&Quote> = self
            .quotes
            .iter()
            .filter(|q| q.length >= min && q.length < max)
            .collect();

        filtered.choose(&mut rand::rng()).copied()
    }

    #[allow(dead_code)]
    pub fn get_quote_by_id(&self, id: usize) -> Option<&Quote> {
        self.quotes.iter().find(|q| q.id == id)
    }

    #[allow(dead_code)]
    pub fn count_by_mode(&self, mode: QuoteMode) -> usize {
        let (min, max) = mode.length_range();
        self.quotes
            .iter()
            .filter(|q| q.length >= min && q.length < max)
            .count()
    }
}

impl Default for QuoteManager {
    fn default() -> Self {
        Self::new().expect("Failed to load quotes")
    }
}
