use std::time::Instant;

#[derive(Debug, Clone)]
pub struct SimpleTimer {
    start: Instant,
}

impl SimpleTimer {
    pub fn start() -> SimpleTimer {
        SimpleTimer {
            start: Instant::now()
        }
    }

    pub fn split(&mut self, text: &str) {
        println!("{}: {:?}", text, self.start.elapsed());
        self.start = Instant::now();
    }
}