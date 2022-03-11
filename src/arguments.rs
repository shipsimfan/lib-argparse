use crate::argument_class::ArgumentClass;
use std::collections::HashMap;

pub trait Get<T: Sized> {
    fn get<S: AsRef<str>>(&self, name: S) -> T {
        match self.get_opt(&name) {
            Some(val) => val,
            None => panic!("Invalid required argument name {}", name.as_ref()),
        }
    }

    fn get_opt<S: AsRef<str>>(&self, name: S) -> Option<T>;
}

pub struct Arguments {
    variables: HashMap<String, ArgumentClass>,
}

pub fn new(variables: HashMap<String, ArgumentClass>) -> Arguments {
    Arguments { variables }
}

impl<'a> Get<bool> for &'a Arguments {
    fn get<S: AsRef<str>>(&self, name: S) -> bool {
        match self.get_opt(name) {
            Some(val) => val,
            None => false,
        }
    }

    fn get_opt<S: AsRef<str>>(&self, name: S) -> Option<bool> {
        match self.variables.get(name.as_ref()) {
            Some(output) => match output {
                ArgumentClass::Boolean => Some(true),
                _ => panic!(
                    "Invalid argument class for {}! Expected a boolean and instead found {}",
                    name.as_ref(),
                    output
                ),
            },
            None => None,
        }
    }
}

impl<'a> Get<&'a str> for &'a Arguments {
    fn get_opt<S: AsRef<str>>(&self, name: S) -> Option<&'a str> {
        let vec: Option<&[String]> = self.get_opt(name);

        vec.map(|vec| {
            if vec.len() != 1 {
                panic!("More than one item available from argument");
            }
            vec[0].as_str()
        })
    }
}

impl<'a> Get<&'a [String]> for &'a Arguments {
    fn get_opt<S: AsRef<str>>(&self, name: S) -> Option<&'a [String]> {
        match self.variables.get(name.as_ref()) {
            Some(output) => match output {
                ArgumentClass::String(vec) => Some(vec.as_slice()),
                _ => panic!(
                    "Invalid argument class for {}! Expected a string array and instead found {}",
                    name.as_ref(),
                    output
                ),
            },
            None => None,
        }
    }
}

impl<'a> Get<usize> for &'a Arguments {
    fn get_opt<S: AsRef<str>>(&self, name: S) -> Option<usize> {
        let vec: Option<&[usize]> = self.get_opt(name);

        vec.map(|vec| {
            if vec.len() != 1 {
                panic!("More than one item available from argument");
            }
            vec[0]
        })
    }
}

impl<'a> Get<&'a [usize]> for &'a Arguments {
    fn get_opt<S: AsRef<str>>(&self, name: S) -> Option<&'a [usize]> {
        match self.variables.get(name.as_ref()) {
            Some(output) => match output {
                ArgumentClass::Integer(vec) => Some(vec.as_slice()),
                _ => panic!(
                    "Invalid argument class for {}! Expected an integer array and instead found {}",
                    name.as_ref(),
                    output
                ),
            },
            None => None,
        }
    }
}

impl<'a> Get<f64> for &'a Arguments {
    fn get_opt<S: AsRef<str>>(&self, name: S) -> Option<f64> {
        let vec: Option<&[f64]> = self.get_opt(name);

        vec.map(|vec| {
            if vec.len() != 1 {
                panic!("More than one item available from argument");
            }
            vec[0]
        })
    }
}

impl<'a> Get<&'a [f64]> for &'a Arguments {
    fn get_opt<S: AsRef<str>>(&self, name: S) -> Option<&'a [f64]> {
        match self.variables.get(name.as_ref()) {
            Some(output) => match output {
                ArgumentClass::Float(vec) => Some(vec.as_slice()),
                _ => panic!(
                    "Invalid argument class for {}! Expected a float array and instead found {}",
                    name.as_ref(),
                    output
                ),
            },
            None => None,
        }
    }
}
