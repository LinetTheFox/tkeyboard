use dirs::home_dir;
use std::fs::read_to_string;

pub struct Generator {
    padded_words: String,
    padded_words_size: usize,
    line_len: usize,
}

impl Generator {
    pub fn init() -> Generator {
        let home_dir_path = home_dir().unwrap();
        let home_dir_string = home_dir_path.to_str().unwrap();
        let mut file_path = String::from(home_dir_string);
        file_path.push_str("/.config/tkeyboard/words_padded.txt");

        let padded_words = read_to_string(String::from(file_path)).unwrap();
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
