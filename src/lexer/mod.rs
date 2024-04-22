pub mod token;

use self::token::{Parse, Token};

pub type Span = std::ops::Range<usize>;

pub struct Lexer {
    pub characters: Vec<(usize, char)>,
    pub current: Option<(usize, char)>,
    pub index: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        let characters = source.char_indices().collect::<Vec<_>>();
        let current = characters.first().cloned();

        Self {
            characters,
            current,
            index: 0,
        }
    }

    pub fn run(mut self) -> Vec<Token> {
        let mut tokens = vec![];
        self.prepare();
        self.step();

        while let Some((i, char)) = self.current {
            // Initiate parsing of token variant
            match char {
                _ if char.is_whitespace() => {}
                char => {
                    let mut data = vec![(i, char)];
                    loop {
                        self.step();
                        if let Some((i, char)) = self.current {
                            if char.is_whitespace() {
                                break;
                            }

                            data.push((i, char));
                        }
                    }

                    Token::parse(&mut tokens, &data).unwrap();
                }
            }

            self.step();
        }

        tokens
    }

    pub fn prepare(&mut self) {
        let mut inside_string = false;

        // ensure nothing cursed happens when we're applying padding, if the condition that the final character in the collection is a delimiter
        self.characters.push((0, '\u{A0}'));

        while let Some((_, char)) = self.current {
            // toggle inside_string
            if char == '"'
                && ((inside_string && self.peek_back().map(|&(_, char)| char) != Some('\\'))
                    || !inside_string)
            {
                inside_string = !inside_string;
            }

            if !inside_string && needs_padding(char) {
                // applying padding before delimiter
                if self
                    .peek_back()
                    .map(|(_, char)| !char.is_whitespace())
                    .unwrap_or(false)
                {
                    self.insert(self.index - 1, (0, ' '));
                    self.index += 1;
                }

                // applying padding after delimiter
                if self
                    .peek()
                    .map(|(_, char)| !char.is_whitespace())
                    .unwrap_or(false)
                {
                    self.insert(self.index, (0, ' '));
                }
            }

            self.step();
        }

        self.reset();
    }

    #[inline]
    fn step(&mut self) {
        self.current = self.characters.get(self.index).cloned();
        self.index += 1;
    }

    #[inline]
    fn reset(&mut self) {
        self.current = None;
        self.index = 0;
    }

    // TODO: extend with blanks if index is beyond the length of the collection
    #[inline]
    fn insert(&mut self, index: usize, value: (usize, char)) {
        self.characters.insert(index, value);
    }

    #[inline]
    fn peek(&self) -> Option<&(usize, char)> {
        self.characters.get(self.index + 1)
    }

    #[inline]
    fn peek_back(&self) -> Option<&(usize, char)> {
        self.characters.get(self.index - 1)
    }
}

fn needs_padding(char: char) -> bool {
    crate::match_one!(char, '(', ')', '[', ']', '{', '}', ',', ';', ':', '*')
}

#[cfg(test)]
mod test {
    use super::{Lexer, Token};

    const TEST_DATA: &'static str = "void fn hello(void);\nchar* temp = \"hello\";";

    #[test]
    fn prepare_works() {
        // TODO: fix this later
        // NOTE: there shouldn't be a space after the first semicolon, but there is in the actual
        const EXPECTED: &'static str = "void fn hello ( void ) ;\nchar* temp = \"hello\" ;\u{A0}";

        let mut lexer = Lexer::new(TEST_DATA);
        lexer.prepare();

        let actual = lexer
            .characters
            .into_iter()
            .map(|(_, char)| char)
            .collect::<String>();
        assert_eq!(EXPECTED, actual);
    }

    #[test]
    fn run_works() {
        const EXPECTED: Vec<Token> = vec![];

        let actual = Lexer::new(TEST_DATA).run();
        println!("{actual:#?}");
        assert_eq!(EXPECTED, actual);
    }
}
