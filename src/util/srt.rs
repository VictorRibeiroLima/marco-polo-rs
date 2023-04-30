use chrono::{Duration, NaiveTime};

use crate::internals::transcriber::traits::Sentence;

pub fn create_based_on_sentences(sentences: Vec<Sentence>) -> String {
    let mut srt_file_raw = String::new();

    for (index, sentence) in sentences.into_iter().enumerate() {
        let start_time = sentence.start_time;
        let end_time = sentence.end_time;
        let text = sentence.text;

        let index = format!("{}\n", index + 1);
        srt_file_raw.push_str(&index);

        let start_time = format_milliseconds(start_time as u32);
        let end_time = format_milliseconds(end_time as u32);
        let time = format!("{} --> {}\n", start_time, end_time);
        srt_file_raw.push_str(&time);

        let text = format!("{}\n\n", text);
        srt_file_raw.push_str(&text);
    }

    return srt_file_raw;
}

fn format_milliseconds(ms: u32) -> String {
    let duration = Duration::milliseconds(ms as i64);
    let time = NaiveTime::from_hms_opt(0, 0, 0).unwrap() + duration;
    return time.format("%H:%M:%S,%3f").to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_milliseconds() {
        let ms = 1000;
        let formatted = format_milliseconds(ms);
        assert_eq!(formatted, "00:00:01,000");
    }
}
