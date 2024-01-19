#[derive(Default)]
pub struct Stats {
    char_count: f64,
    seconds: f64,
    mistakes: f64,
}

impl Stats {
    pub fn add(&mut self, stats: Stats) {
        self.char_count += stats.char_count;
        self.seconds += stats.seconds;
        self.mistakes += stats.mistakes;
    }

    pub fn chars_per_minute(&self) -> f64 {
        ((self.char_count - self.mistakes) / self.seconds * 60.0).round()
    }

    pub fn words_per_minute(&self) -> f64 {
        (self.chars_per_minute() / 5.0).round()
    }

    pub fn accuracy(&self) -> f64 {
        (1.0 - (self.mistakes / self.char_count)).round()
    }
}
