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
        l.read_char();
        l
    }

    pub fn read_char(&mut self) {
        if self.readposition >= self.input.len() {
            self.ch = None;
        } else {
            let ch = self.input[self.readposition..].chars().next();
            self.position = Some(self.readposition);

            if let Some(character) = ch {
                self.readposition += character.len_utf8();
            }

            self.ch = ch;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_initialization() {
        let input = String::from("hello");
        let lexer = Lexer::new(input.clone());

        assert_eq!(lexer.input, input);
        assert_eq!(lexer.position, Some(0));
        assert_eq!(lexer.readposition, 1);
        assert_eq!(lexer.ch, Some('h'));
    }

    #[test]
    fn test_read_char() {
        let input = String::from("hell😃o");
        let mut lexer = Lexer::new(input.clone());

        lexer.read_char();
        assert_eq!(lexer.position, Some(1));
        assert_eq!(lexer.ch, Some('e'));

        lexer.read_char();
        assert_eq!(lexer.position, Some(2));
        assert_eq!(lexer.ch, Some('l'));

        lexer.read_char();
        assert_eq!(lexer.position, Some(3));
        assert_eq!(lexer.ch, Some('l'));

        lexer.read_char();
        assert_eq!(lexer.position, Some(4));
        assert_eq!(lexer.ch, Some('😃'));

        lexer.read_char();
        assert_eq!(lexer.position, Some(8));
        assert_eq!(lexer.ch, Some('o'));

        lexer.read_char();
        assert_eq!(lexer.position, Some(8));
        assert_eq!(lexer.readposition, 9);
        assert_eq!(lexer.ch, None);
    }
}
