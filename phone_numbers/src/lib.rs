use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

#[derive(Default)]
pub struct PhoneEncoder {
    phone_trie: PhoneTrie,
}

impl PhoneEncoder {
    pub fn put_word(&mut self, word: String) {
        let canonized_word: Vec<char> = word
            .chars()
            .filter_map(|c| {
                if c.is_alphabetic() {
                    Some(c.to_lowercase().next().unwrap())
                } else {
                    None
                }
            })
            .collect();

        self.phone_trie.put_word(word, &canonized_word, 0);
    }

    pub fn print_phone_encodings(&self, file: &mut File, phone_number: String) {
        let phone_digits: Vec<usize> = phone_number
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as usize))
            .collect();

        let mut encodings: Vec<Vec<&WordInfo>> = vec![vec![]; phone_digits.len()];
        for (i, encoding) in encodings.iter_mut().enumerate() {
            self.phone_trie.get_words(&phone_digits, i, encoding);
        }
        PhoneEncoder::print_phones(
            file,
            &EncodingData {
                phone_number,
                phone_digits,
                encodings,
            },
            &mut vec![],
            0,
            true,
        );
    }

    pub fn encode_phones<P: AsRef<Path>>(dictionary: P, input: P, output: P) {
        let mut phone_encoder = PhoneEncoder::default();

        let dictionary_lines =
            BufReader::new(File::open(dictionary).expect("Failed to open dictionary file")).lines();
        let input_lines =
            BufReader::new(File::open(input).expect("Failed to open input file")).lines();
        let mut output_file = File::create(output).expect("Failed to create output file");

        for word in dictionary_lines {
            phone_encoder.put_word(word.expect("Failed to read word"));
        }
        for phone_number in input_lines {
            phone_encoder.print_phone_encodings(
                &mut output_file,
                phone_number.expect("Failed to read phone number"),
            );
        }
    }

    fn print_phones<'a>(
        file: &mut File,
        encoding_data: &'a EncodingData,
        encoded_prefix: &mut Vec<&'a String>,
        index: usize,
        can_skip_digit: bool,
    ) {
        if index == encoding_data.encodings.len() {
            write!(file, "{}:", encoding_data.phone_number).expect("Failed to write");
            encoded_prefix.iter().for_each(|w| {
                write!(file, " {}", w).expect("Failed to write");
            });
            writeln!(file).expect("Failed to write");
        }
        if index >= encoding_data.encodings.len() {
            return;
        }

        for next_word in &encoding_data.encodings[index] {
            encoded_prefix.push(&next_word.word);
            PhoneEncoder::print_phones(
                file,
                encoding_data,
                encoded_prefix,
                index + next_word.canonized_length,
                true,
            );
            encoded_prefix.pop();
        }

        if encoding_data.encodings[index].is_empty() && can_skip_digit {
            encoded_prefix.push(&DIGIT_TO_STRING[&encoding_data.phone_digits[index]]);
            PhoneEncoder::print_phones(file, encoding_data, encoded_prefix, index + 1, false);
            encoded_prefix.pop();
        }
    }
}

struct WordInfo {
    word: String,
    canonized_length: usize,
}

struct PhoneTrie {
    children: Vec<Option<PhoneTrie>>,
    words: Vec<WordInfo>,
}

impl Default for PhoneTrie {
    fn default() -> Self {
        PhoneTrie {
            children: (0..10).map(|_| None).collect(),
            words: vec![],
        }
    }
}

lazy_static! {
    static ref MATCH_LETTER: HashMap<char, usize> = HashMap::from([
        ('e', 0),
        ('j', 1),
        ('n', 1),
        ('q', 1),
        ('r', 2),
        ('w', 2),
        ('x', 2),
        ('d', 3),
        ('s', 3),
        ('y', 3),
        ('f', 4),
        ('t', 4),
        ('a', 5),
        ('m', 5),
        ('c', 6),
        ('i', 6),
        ('v', 6),
        ('b', 7),
        ('k', 7),
        ('u', 7),
        ('l', 8),
        ('o', 8),
        ('p', 8),
        ('g', 9),
        ('h', 9),
        ('z', 9),
    ]);
    static ref DIGIT_TO_STRING: HashMap<usize, String> =
        HashMap::from_iter((0..10).map(|d| { (d, d.to_string()) }));
}

impl PhoneTrie {
    fn put_word(&mut self, word: String, canonized_word: &[char], depth: usize) {
        if depth == canonized_word.len() {
            self.words.push(WordInfo {
                word,
                canonized_length: canonized_word.len(),
            })
        } else {
            let index = MATCH_LETTER[&canonized_word[depth]];
            if self.children[index].is_none() {
                self.children[index] = Some(PhoneTrie::default());
            }
            self.children[index]
                .as_mut()
                .unwrap()
                .put_word(word, canonized_word, depth + 1);
        }
    }

    fn get_words<'a>(&'a self, phone: &[usize], depth: usize, words: &mut Vec<&'a WordInfo>) {
        words.extend(self.words.iter());
        if depth >= phone.len() {
            return;
        }
        if let Some(ref child) = self.children[phone[depth]] {
            child.get_words(phone, depth + 1, words)
        }
    }
}

struct EncodingData<'a> {
    phone_number: String,
    phone_digits: Vec<usize>,
    encodings: Vec<Vec<&'a WordInfo>>,
}
