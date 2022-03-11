use crate::{argument_class::ArgumentClass, ArgumentParseError};
use class::Class;

mod class;

pub struct MovableArgument {
    names: Vec<String>,
    variable: String,
    hint: Option<String>,
    help: Option<String>,
    class: Class,
    required: bool,
}

impl MovableArgument {
    pub fn new<S: AsRef<str>>(name: S) -> Self {
        MovableArgument {
            names: vec![name.as_ref().to_owned()],
            variable: name.as_ref().to_owned(),
            hint: None,
            help: None,
            class: Class::Boolean,
            required: false,
        }
    }

    pub fn name<S: AsRef<str>>(&mut self, name: S) {
        self.names.push(name.as_ref().to_owned());
        self.sort_names();
    }

    pub fn names<S: AsRef<str>>(&mut self, names: &[S]) {
        self.names
            .extend(names.iter().map(|s| s.as_ref().to_owned()));
        self.sort_names();
    }

    pub fn variable_name<S: AsRef<str>>(&mut self, name: S) {
        self.variable = name.as_ref().to_owned();
    }

    pub fn hint<S: AsRef<str>>(&mut self, hint: S) {
        self.hint = Some(hint.as_ref().to_owned());
    }

    pub fn help<S: AsRef<str>>(&mut self, message: S) {
        self.help = Some(message.as_ref().to_owned());
    }

    pub fn class<S: AsRef<str>>(&mut self, class: S) {
        self.class = Class::new(class);
    }

    pub fn count(&mut self, new_count: usize) {
        self.class.update_count(new_count);
    }

    pub fn required(&mut self, required: bool) {
        self.required = required;
    }

    pub fn sort_names(&mut self) {
        self.names.sort_by(|s1, s2| match s1.len().cmp(&s2.len()) {
            std::cmp::Ordering::Equal => s1.cmp(s2),
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
        })
    }

    pub fn generate_usage(&self) -> String {
        let mut string = format!("{}", self.names[0]);

        match self.class {
            Class::Boolean => return string,
            _ => {}
        }

        string.push(' ');
        string.push_str(&self.generate_hint());

        string
    }

    pub fn generate_hint(&self) -> String {
        match self.class {
            Class::Boolean => return String::new(),
            _ => {}
        }

        match &self.hint {
            Some(hint) => hint.to_owned(),
            None => format!("{}", self.class),
        }
    }

    pub fn has_hint(&self) -> bool {
        match self.class {
            Class::Boolean => false,
            _ => self.hint.is_some(),
        }
    }

    pub fn is_required(&self) -> bool {
        self.required
    }

    pub fn get_names(&self) -> &[String] {
        self.names.as_slice()
    }

    pub fn get_variable_name(&self) -> &str {
        &self.variable
    }

    pub fn get_help(&self) -> &str {
        match &self.help {
            Some(help) => help,
            None => "",
        }
    }

    pub fn name_match<S: AsRef<str>>(&self, name: S) -> bool {
        for inner_name in &self.names {
            if inner_name == name.as_ref() {
                return true;
            }
        }

        false
    }

    pub fn parse<I: Iterator<Item = String>>(
        &self,
        name: &str,
        iter: &mut I,
    ) -> Result<(String, ArgumentClass), ArgumentParseError> {
        Ok((self.variable.clone(), self.class.parse(name, iter)?))
    }
}
