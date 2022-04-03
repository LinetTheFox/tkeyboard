pub struct Settings {
    pub word_count: usize,
}

impl Settings {
    pub fn default() -> Settings {
        Settings { word_count: 20 }
    }
}
