use crate::{argument_class::ArgumentClass, ArgumentParseError};

pub enum Class {
    Boolean,
    String(usize),
    Integer(usize),
    Float(usize),
}

impl Class {
    pub fn new<S: AsRef<str>>(class: S) -> Self {
        match class.as_ref() {
            "boolean" => Class::Boolean,
            "string" => Class::String(1),
            "integer" => Class::Integer(1),
            "float" => Class::Float(1),
            _ => panic!("Invalid class {}", class.as_ref()),
        }
    }

    pub fn update_count(&mut self, new_count: usize) {
        if new_count == 0 {
            panic!("Zero count is not valid for optional arguments");
        }

        match self {
            Class::Boolean => panic!("Cannot set a count for boolean typed arguments"),
            Class::String(count) | Class::Integer(count) | Class::Float(count) => {
                *count = new_count
            }
        }
    }

    pub fn parse<I: Iterator<Item = String>>(
        &self,
        name: &str,
        iter: &mut I,
    ) -> Result<ArgumentClass, ArgumentParseError> {
        Ok(match self {
            Class::Boolean => ArgumentClass::Boolean,
            Class::String(count) => {
                let mut vec = Vec::with_capacity(*count);

                for i in 0..*count {
                    match iter.next() {
                        Some(string) => vec.push(string),
                        None => {
                            return Err(ArgumentParseError::InvalidNumber(
                                name.to_owned(),
                                *count,
                                i,
                            ))
                        }
                    }
                }

                ArgumentClass::String(vec)
            }
            Class::Integer(count) => {
                let mut vec = Vec::with_capacity(*count);

                for i in 0..*count {
                    match iter.next() {
                        Some(string) => match string.parse() {
                            Ok(val) => vec.push(val),
                            Err(_) => return Err(ArgumentParseError::InvalidInteger(string)),
                        },
                        None => {
                            return Err(ArgumentParseError::InvalidNumber(
                                name.to_owned(),
                                *count,
                                i,
                            ))
                        }
                    }
                }

                ArgumentClass::Integer(vec)
            }
            Class::Float(count) => {
                let mut vec = Vec::with_capacity(*count);

                for i in 0..*count {
                    match iter.next() {
                        Some(string) => match string.parse() {
                            Ok(val) => vec.push(val),
                            Err(_) => return Err(ArgumentParseError::InvalidFloat(string)),
                        },
                        None => {
                            return Err(ArgumentParseError::InvalidNumber(
                                name.to_owned(),
                                *count,
                                i,
                            ))
                        }
                    }
                }

                ArgumentClass::Float(vec)
            }
        })
    }
}

impl std::fmt::Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (count, class) = match self {
            Class::Boolean => return Ok(()),
            Class::String(count) => (*count, "STRING"),
            Class::Integer(count) => (*count, "INTEGER"),
            Class::Float(count) => (*count, "FLOAT"),
        };

        if count == 1 {
            write!(f, "{}", class)
        } else {
            write!(f, "{} {}S", count, class)
        }
    }
}
