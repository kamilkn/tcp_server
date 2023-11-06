use rand::seq::SliceRandom;

pub struct QuotesStore {
    quotes: Vec<String>,
}

impl QuotesStore {
    pub fn new(quotes: Vec<String>) -> QuotesStore {
      QuotesStore { quotes }
    }

    pub fn get_random_quote(&self) -> Option<&String> {
        self.quotes.choose(&mut rand::thread_rng())
    }
}
