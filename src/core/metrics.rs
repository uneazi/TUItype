use std::time::Instant;

/// Calculate WPM (Words Per Minute) based on characters typed and elapsed time
pub fn calculate_wpm(chars_typed: usize, elapsed_secs: f64) -> f64 {
    if elapsed_secs < 1.0 / 60.0 {
        return 0.0;
    }
    let words = chars_typed as f64 / 5.0;
    words / (elapsed_secs / 60.0)
}

/// Calculate raw WPM (including mistakes)
pub fn calculate_raw_wpm(total_chars: usize, elapsed_secs: f64) -> f64 {
    if elapsed_secs < 1.0 / 60.0 {
        return 0.0;
    }
    let words = total_chars as f64 / 5.0;
    words / (elapsed_secs / 60.0)
}

/// Calculate accuracy percentage
pub fn calculate_accuracy(correct: usize, attempted: usize) -> f64 {
    if attempted == 0 {
        return 100.0;
    }
    (correct as f64 / attempted as f64) * 100.0
}

/// Count correct characters in typed text against quote
pub fn count_correct_chars(typed: &str, quote: &str) -> usize {
    typed
        .chars()
        .enumerate()
        .filter(|(i, ch)| quote.chars().nth(*i) == Some(*ch))
        .count()
}

/// Calculate WPM consistency from history
pub fn calculate_consistency(wpm_history: &[(Instant, f64)]) -> f64 {
    if wpm_history.len() < 2 {
        return 100.0;
    }

    let wpms: Vec<f64> = wpm_history.iter().map(|(_, wpm)| *wpm).collect();
    let mean = wpms.iter().sum::<f64>() / wpms.len() as f64;
    let variance = wpms.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / wpms.len() as f64;
    let std_dev = variance.sqrt();

    // Convert to percentage (lower std_dev = higher consistency)
    ((mean - std_dev) / mean * 100.0).max(0.0).min(100.0)
}

/// Animate WPM value towards target
pub fn animate_wpm(current: f64, target: f64, last_for_animation: &mut f64) -> f64 {
    if target == 0.0 {
        *last_for_animation = 0.0;
        return 0.0;
    }

    if (target - *last_for_animation).abs() < 0.5 {
        return current;
    }

    let diff = target - *last_for_animation;
    let new_value = current + diff * 0.15;
    *last_for_animation = new_value;

    if diff.abs() < 0.1 {
        *last_for_animation = target;
        return target;
    }

    new_value
}
