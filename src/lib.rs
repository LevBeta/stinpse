use std::{
    str::Chars,
    iter::Peekable,
};

enum State {
    Normal,
    InSingleQuote,
    InDoubleQuote,
}

/// A Parser struct that holds the context for operations
pub struct Parser<'a> {
    /// Parsed arguments
    args: Vec<String>,
    /// Current argument
    current: String,
    /// Current state of the parser
    state: State,
    /// Iter over the characters of the input string
    chars: Peekable<Chars<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let chars = input.chars().peekable();

        Parser {
            args: Vec::new(),
            current: String::new(),
            state: State::Normal,
            chars,
        }
    }

    pub fn parse(input: &'a str) -> Self {
        let mut parser = Self::new(input);

        while let Some(char) = parser.chars.next() {
            match parser.state {
                State::Normal => parser.handle_normal(char),
                State::InSingleQuote => parser.handle_in_single_quote(char),
                State::InDoubleQuote => parser.handle_in_double_quotes(char),
            }
        }

        if !parser.current.is_empty() {
            parser.args.push(parser.current.clone());
        }

        parser
    }

    pub fn get_args(&self) -> &Vec<String> {
        &self.args
    }

    fn handle_normal(&mut self, char: char) {
        match char {
            '\'' => self.state = State::InSingleQuote,
            '"' => self.state = State::InDoubleQuote,
            ' ' => {
                if !self.current.is_empty() {
                    self.args.push(self.current.clone());
                    self.current.clear();
                }
            },
            _ => self.current.push(char),
        }
    }

    fn handle_in_single_quote(&mut self, char: char) {
        match char {
            '\'' => self.state = State::Normal,
            _ => self.current.push(char),
        }
    }
    
    fn handle_in_double_quotes(&mut self, char: char) {
        match char {
            '"' => self.state = State::Normal,
            '\\' => {
                if let Some(next_char) = self.chars.peek() {
                    if next_char == &'"' || next_char == &'\\' {
                        self.current.push(self.chars.next().unwrap());
                    }
                }
            },
            _ => self.current.push(char),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn match_result(test_number: usize, input: &str, expected: Vec<&str>) {
        let parser = Parser::parse(input);
        assert_eq!(parser.args, expected, "Test case {} failed: input = {}, expected = {:?}, actual = {:?}", test_number, input, expected, parser.args);
    }

    #[test]
    fn test_parser() {
        let test_cases = vec![
            // Undefined number of arguments
            ("ls -l", vec!["ls", "-l"]),
            ("ls -l -a", vec!["ls", "-l", "-a"]),
            ("ls -l -a -h", vec!["ls", "-l", "-a", "-h"]),
            ("ls -l -a -h -b", vec!["ls", "-l", "-a", "-h", "-b"]),
            ("ls -l -a -h -b -c", vec!["ls", "-l", "-a", "-h", "-b", "-c"]),
            ("ls -l -a -h -b -c -d", vec!["ls", "-l", "-a", "-h", "-b", "-c", "-d"]),
            // Single quotes
            ("ls 'ls -l'", vec!["ls", "ls -l"]),
            ("ls 'ls -l' -a", vec!["ls", "ls -l", "-a"]),
            ("ls 'ls -l' -a -h", vec!["ls", "ls -l", "-a", "-h"]),
            ("ls 'ls -l' -a -h -b", vec!["ls", "ls -l", "-a", "-h", "-b"]),
            ("ls 'ls -l' -a -h -b -c", vec!["ls", "ls -l", "-a", "-h", "-b", "-c"]),
            // Double quotes
            ("ls \"ls -l\"", vec!["ls", "ls -l"]),
            ("ls \"ls -l\" -a", vec!["ls", "ls -l", "-a"]),
            ("ls \"ls -l\" -a -h", vec!["ls", "ls -l", "-a", "-h"]),
            ("ls \"ls -l\" -a -h -b", vec!["ls", "ls -l", "-a", "-h", "-b"]),
            ("ls \"ls -l\" -a -h -b -c", vec!["ls", "ls -l", "-a", "-h", "-b", "-c"]),
            
            
        ];

        for (i, (input, expected)) in test_cases.iter().enumerate() {
            match_result(i + 1, input, expected.to_vec());
        }
    }
}
