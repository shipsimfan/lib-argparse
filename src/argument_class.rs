use crate::ArgumentParseError;

pub enum ArgumentClass {
    Boolean,
    String(Vec<String>),
    Integer(Vec<usize>),
    Float(Vec<f64>),
}

impl ArgumentClass {
    pub fn insert(&mut self, string: String) -> Result<(), ArgumentParseError> {
        match self {
            ArgumentClass::Boolean => panic!("Cannot insert an element into a boolean argument"),
            ArgumentClass::String(vec) => vec.push(string),
            ArgumentClass::Integer(vec) => match string.parse() {
                Ok(val) => vec.push(val),
                Err(_) => return Err(ArgumentParseError::InvalidInteger(string)),
            },
            ArgumentClass::Float(vec) => match string.parse() {
                Ok(val) => vec.push(val),
                Err(_) => return Err(ArgumentParseError::InvalidFloat(string)),
            },
        }

        Ok(())
    }

    pub fn len(&self) -> usize {
        match self {
            ArgumentClass::Boolean => panic!("Cannot get the length of a boolean argument"),
            ArgumentClass::Float(vec) => vec.len(),
            ArgumentClass::Integer(vec) => vec.len(),
            ArgumentClass::String(vec) => vec.len(),
        }
    }

    pub fn extend(&mut self, other: ArgumentClass) {
        match self {
            ArgumentClass::Boolean => panic!("Cannot have multiple booleans"),
            ArgumentClass::String(vec) => match other {
                ArgumentClass::String(vec2) => vec.extend(vec2.into_iter()),
                _ => panic!("Type mismatch"),
            },
            ArgumentClass::Integer(vec) => match other {
                ArgumentClass::Integer(vec2) => vec.extend(vec2.into_iter()),
                _ => panic!("Type mismatch"),
            },
            ArgumentClass::Float(vec) => match other {
                ArgumentClass::Float(vec2) => vec.extend(vec2.into_iter()),
                _ => panic!("Type mismatch"),
            },
        }
    }
}

impl std::fmt::Display for ArgumentClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ArgumentClass::Boolean => "a boolean",
                ArgumentClass::String(_) => "a string",
                ArgumentClass::Integer(_) => "an integer",
                ArgumentClass::Float(_) => "a float",
            }
        )
    }
}
