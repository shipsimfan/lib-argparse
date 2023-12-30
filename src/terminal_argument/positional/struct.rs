use crate::{Error, PositionalArgument, TerminalArgument};

/// A series of positional arguments
pub struct PositionalTerminalArgument<'a, Options: 'a> {
    header: Option<&'a dyn std::fmt::Display>,

    positionals: &'a [&'a dyn PositionalArgument<'a, Options>],
    collecting: Option<&'a dyn PositionalArgument<'a, Options>>,
}

impl<'a, Options> PositionalTerminalArgument<'a, Options> {
    /// Creates a new [`PositionalTerminalArgument`]
    pub const fn new() -> Self {
        PositionalTerminalArgument {
            header: None,
            positionals: &[],
            collecting: None,
        }
    }

    /// Sets the header for these positionals
    pub const fn header(mut self, header: &'a dyn std::fmt::Display) -> Self {
        self.header = Some(header);
        self
    }

    /// Sets the positional arguments
    pub const fn positionals(
        mut self,
        positionals: &'a [&'a dyn PositionalArgument<'a, Options>],
    ) -> Self {
        self.positionals = positionals;
        self
    }

    /// Sets the collecting positional, which will recieve all remaining arguments after the last
    /// normal positional
    pub const fn collecting(mut self, collecting: &'a dyn PositionalArgument<'a, Options>) -> Self {
        self.collecting = Some(collecting);
        self
    }
}

impl<'a, Options> TerminalArgument<'a, Options> for PositionalTerminalArgument<'a, Options> {
    fn action(
        &self,
        options: &mut Options,
        mut index: usize,
        parameter: std::ffi::OsString,
    ) -> crate::Result<'a, Option<&crate::Parser<'a, Options>>> {
        for positional in self.positionals {
            if positional.count().get() > index {
                return positional.action(options, index, parameter).map(|_| None);
            } else {
                index -= positional.count().get();
            }
        }

        if let Some(collecting) = self.collecting {
            collecting.action(options, index, parameter).map(|_| None)
        } else {
            Err(Error::unexpected_argument(format!(
                "unexpected argument \"{}\"",
                parameter.to_string_lossy()
            )))
        }
    }

    fn finalize(&self, mut count: usize) -> crate::Result<'a, ()> {
        for positional in self.positionals {
            let positional_count = if positional.count().get() > count {
                let positional_count = count;
                count = 0;
                positional_count
            } else {
                count -= positional.count().get();
                positional.count().get()
            };

            positional.finalize(positional_count)?;
        }

        if let Some(collecting) = self.collecting {
            collecting.finalize(count)?;
        }

        Ok(())
    }

    fn help(&self, f: &mut dyn std::fmt::Write) -> std::fmt::Result {
        let mut longest_name = 0;
        for positional in self.positionals {
            let length = positional.name().to_string().len();
            if length > longest_name {
                longest_name = length;
            }
        }

        if let Some(collecting) = self.collecting {
            let length = collecting.name().to_string().len();
            if length > longest_name {
                longest_name = length;
            }
        }

        write!(f, "{}:", self.header.unwrap_or(&"ARGUMENTS"))?;
        for positional in self.positionals {
            write!(f, "  {}", positional.name())?;

            for _ in positional.name().to_string().len()..longest_name {
                write!(f, " ")?;
            }

            let description = positional.description();
            if description.len() == 0 {
                writeln!(f)?;
                continue;
            }

            writeln!(f, "  {}", description[0])?;
            for i in 1..description.len() {
                for _ in 0..longest_name + 4 {
                    write!(f, " ")?;
                }

                writeln!(f, "{}", description[i])?;
            }
        }

        if let Some(collecting) = self.collecting {
            write!(f, "  {}", collecting.name())?;

            for _ in collecting.name().to_string().len()..longest_name {
                write!(f, " ")?;
            }

            let description = collecting.description();
            if description.len() == 0 {
                writeln!(f)?;
                return Ok(());
            }

            writeln!(f, "  {}", description[0])?;
            for i in 1..description.len() {
                for _ in 0..longest_name + 4 {
                    write!(f, " ")?;
                }

                writeln!(f, "{}", description[i])?;
            }
        }

        Ok(())
    }

    fn hint(&self) -> &dyn std::fmt::Display {
        self
    }
}

impl<'a, Options> std::fmt::Display for PositionalTerminalArgument<'a, Options> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for positional in self.positionals {
            if let Some(hint) = positional.hint() {
                if first {
                    first = false;
                } else {
                    write!(f, " ")?;
                }

                write!(f, "{}", hint)?;
            }
        }

        if let Some(collecting) = self.collecting {
            if let Some(hint) = collecting.hint() {
                if !first {
                    write!(f, " ")?;
                }

                write!(f, "{}", hint)
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }
}
