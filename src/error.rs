pub struct ErrorReporter {
    had_error: bool,
}

impl ErrorReporter {
    pub fn new() -> Self {
        Self { had_error: false }
    }

    pub fn error(&mut self, line: usize, msg: &str) {
        self.report(line, "", msg);
    }

    pub fn report(&mut self, line: usize, loc: &str, msg: &str) {
        println!("[line {}] Error {}: {}.", line, loc, msg);
        self.had_error = true;
    }
}