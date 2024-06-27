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
        let mut quoted = first_char == '"';

        let mut string = if !quoted {
            first_char.to_string()
        } else {
            String::new()
        };

        // Parse the stream
        while let Some(c) = self.next_char()? {
            if quoted {
                if c == '\\' {
                    let c = match self.next_char()? {
                        Some(c) => c,
                        None => return Err(Error::unexpected_end_of_stream()),
                    };

                    if c != '"' {
                        string.push('\\');
                    }

                    string.push(c);
                    continue;
                } else if c == '"' {
                    quoted = false;
                    continue;
                }

                string.push(c);
            } else {
                if c.is_whitespace() {
                    break;
                }

                string.push(c);
            }
        }

        Ok(Some(string))
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
