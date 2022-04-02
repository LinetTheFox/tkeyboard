use std::fs::read_to_string;

pub struct Generator {
    padded_words: String,
    padded_words_size: usize,
    line_len: usize,
}

impl Generator {
    pub fn init(file_path: &str) -> Generator {
        let padded_words = read_to_string(file_path).unwrap();
        let padded_words_size = padded_words.len();
        // including '\n' at the end
        let line_len = padded_words.chars().take_while(|c| *c != '\n').count() + 1;

        Generator {
            padded_words: padded_words,
            padded_words_size: padded_words_size,
            line_len: line_len,
        }
    }

    pub fn generate(&self, word_count: u64) -> String {
        let mut res = String::from("");
        let max = self.padded_words_size / self.line_len;
        for _ in 0..word_count {
            let start = rand::random::<usize>() % max * self.line_len;
            let end = start as usize + self.line_len;
            let splice = self.padded_words.get(start..end).unwrap();
            res.push_str(splice.trim());
            res.push_str(" ");
        }
        res.trim_end().to_string()
    }
}
