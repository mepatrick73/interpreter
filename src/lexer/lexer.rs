struct Lexer {
    pub input: String,
    position: Option<usize>,
    readposition: usize,
    ch: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Lexer {
            input,
            position: None,
            readposition: 0,
            ch: None,
        };
        l.readChar();
        l
    }

    pub fn readChar(&mut self) {
        if self.readposition > self.input.len() {
            self.ch = None;
        } else {
            let ch = self.input[self.readposition..].chars().next();
            self.position = Some(self.readposition);
            self.readposition += ch.map(|_| 1).unwrap_or(0);
            self.ch = ch;
        }
    }
}
