use crate::{Error, Result};
use std::io::{BufReader, Bytes, Read};

/// Parses a [`Read`] into arguments
pub(super) struct IOArgumentParser<'a> {
    /// The source for reading
    source: Bytes<BufReader<&'a mut dyn Read>>,
}

impl<'a> IOArgumentParser<'a> {
    /// Creates a new [`IOArgumentParser`] for `source`
    pub fn new(source: &'a mut dyn Read) -> Self {
        let source = BufReader::new(source).bytes();

        IOArgumentParser { source }
    }

    /// Gets the next argument in the stream
    pub(super) fn next(&mut self) -> Result<'static, Option<String>> {
        // Skip whitespace
        let mut first_char = None;
        while let Some(c) = self.next_char()? {
            if !c.is_whitespace() {
                first_char = Some(c);
                break;
            }
        }

        let first_char = match first_char {
            Some(first_char) => first_char,
            None => return Ok(None), // Reached the end of the stream
        };

        // Check if starts with '"'
        if first_char == '"' {
            self.collect_quoted_argument()
        } else {
            self.collect_unquoted_argument(first_char)
        }
        .map(|string| Some(string))
    }

    /// Collects the characters until an unescaped quote is found, erroring if none are found
    fn collect_quoted_argument(&mut self) -> Result<'static, String> {
        let mut string = String::new();

        while let Some(c) = self.next_char()? {
            if c == '\\' {
                let c = match self.next_char()? {
                    Some(c) => c,
                    None => return Err(Error::unexpected_end_of_stream()),
                };

                string.push(c);
            }

            if c == '"' {
                return Ok(string);
            }
        }

        Err(Error::unexpected_end_of_stream())
    }

    /// Collects the characters until a whitespace or the end of the stream is found
    fn collect_unquoted_argument(&mut self, first_char: char) -> Result<'static, String> {
        let mut string = first_char.to_string();

        while let Some(c) = self.next_char()? {
            if c.is_whitespace() {
                break;
            }

            string.push(c);
        }

        Ok(string)
    }

    /// Gets the next character in the stream
    fn next_char(&mut self) -> Result<'static, Option<char>> {
        let mut char = 0;

        for i in 0..4 {
            let byte = match self.source.next() {
                Some(c) => c,
                None => match i {
                    0 => return Ok(None),
                    _ => break,
                },
            }?;

            char <<= 8;
            char |= byte as u32;

            if let Some(c) = char::from_u32(char) {
                return Ok(Some(c));
            }
        }

        Err(Error::invalid_utf8())
    }
}
