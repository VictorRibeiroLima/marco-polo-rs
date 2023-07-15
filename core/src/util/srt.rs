use chrono::{Duration, NaiveTime};

use crate::internals::transcriber::traits::Sentence;

const MAX_SENTENCE_LENGTH: usize = 110; // including spaces

pub fn create_based_on_sentences(sentences: Vec<Sentence>) -> String {
    let mut srt_file_raw = String::new();

    let sentences: Vec<Sentence> = sentences
        .into_iter()
        .flat_map(|sentence| split_sentence(sentence))
        .collect();

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

fn split_sentence(sentence: Sentence) -> Vec<Sentence> {
    let mut new_sentences: Vec<Sentence> = vec![];
    let text = sentence.text;
    let start_time = sentence.start_time;
    let end_time = sentence.end_time;
    let text_len = text.len();

    if text_len <= MAX_SENTENCE_LENGTH {
        new_sentences.push(Sentence {
            text,
            start_time,
            end_time,
        });
        return new_sentences;
    }

    let mut sentences: Vec<Sentence> = vec![];
    let time_diff = end_time - start_time;
    let half_time = time_diff / 2;

    let text = text.split_whitespace().collect::<Vec<&str>>();
    let middle = text.len() / 2;
    let first_half = text[..middle].join(" ");
    let second_half = text[middle..].join(" ");

    let first_half_sentence = Sentence {
        text: first_half,
        start_time,
        end_time: start_time + half_time,
    };

    let second_half_sentence = Sentence {
        text: second_half,
        start_time: start_time + half_time,
        end_time,
    };

    let first_half_sentences = split_sentence(first_half_sentence);
    let second_half_sentences = split_sentence(second_half_sentence);

    sentences.extend(first_half_sentences);
    sentences.extend(second_half_sentences);

    return sentences;
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

    #[test]
    fn test_split_sentence() {
        let sentence = Sentence {
            text: "Ele é capaz de decodificar, codificar, transcodificar, multiplexar D, multiplexar fluxo, filtrar e reproduzir praticamente qualquer arquivo multimídia do mundo, com suporte a mais de 100 codecs diferentes.".to_string(),
            start_time: 0,
            end_time: 1000,
        };

        let sentences = split_sentence(sentence);
        assert_eq!(sentences.len(), 2);
        let first_sentence = &sentences[0];
        let second_sentence = &sentences[1];

        assert!(first_sentence.text.len() <= MAX_SENTENCE_LENGTH);
        assert!(second_sentence.text.len() <= MAX_SENTENCE_LENGTH);

        assert_eq!(first_sentence.start_time, 0);
        assert_eq!(first_sentence.end_time, 500);

        assert_eq!(second_sentence.start_time, 500);
        assert_eq!(second_sentence.end_time, 1000);
    }

    #[test]
    fn test_srt() {
        let expected_text: &str = "1\n00:00:01,370 --> 00:00:02,654\nSua vida não é nada.\n\n2\n00:00:02,772 --> 00:00:04,750\nVocê não serve para nada.\n\n3\n00:00:05,170 --> 00:00:10,438\nVocê deveria se matar agora e dar a outra pessoa um pedaço da camada\n\n4\n00:00:10,438 --> 00:00:15,706\nde oxigênio e ozônio que está encoberta para que possamos respirar dentro dessa bolha azul.\n\n5\n00:00:15,818 --> 00:00:17,150\nPorque você está aqui para quê?\n\n6\n00:00:17,220 --> 00:00:17,340\nPara.\n\n";

        let senteces = vec![
            Sentence {
                text: "Sua vida não é nada.".to_string(),
                start_time: 1370,
                end_time: 2654,
            },
            Sentence {
                text: "Você não serve para nada.".to_string(),
                start_time: 2772,
                end_time: 4750,
            },
            Sentence {
                text: "Você deveria se matar agora e dar a outra pessoa um pedaço da camada de oxigênio e ozônio que está encoberta para que possamos respirar dentro dessa bolha azul.".to_string(),
                start_time: 5170,
                end_time: 15706,
            },
            Sentence {
                text: "Porque você está aqui para quê?".to_string(),
                start_time: 15818,
                end_time: 17150,
            },
            Sentence {
                text: "Para.".to_string(),
                start_time: 17220,
                end_time: 17340,
            },
        ];

        let srt = create_based_on_sentences(senteces);
        assert_eq!(srt, expected_text);
    }
}
