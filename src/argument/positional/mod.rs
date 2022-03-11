use class::Class;

use crate::argument_class::ArgumentClass;

mod class;

pub struct PositionalArgument {
    name: String,
    variable: String,
    help: Option<String>,
    class: Class,
}

impl PositionalArgument {
    pub fn new<S: AsRef<str>>(name: S) -> Self {
        PositionalArgument {
            name: name.as_ref().to_owned(),
            variable: name.as_ref().to_owned(),
            help: None,
            class: Class::new("string"),
        }
    }

    pub fn variable_name<S: AsRef<str>>(&mut self, name: S) {
        self.variable = name.as_ref().to_owned();
    }

    pub fn help<S: AsRef<str>>(&mut self, message: S) {
        self.help = Some(message.as_ref().to_owned());
    }

    pub fn class<S: AsRef<str>>(&mut self, class: S) {
        self.class = Class::new(class);
    }

    pub fn count(&mut self, new_count: usize) {
        self.maximum(new_count);
    }

    pub fn maximum(&mut self, maximum: usize) {
        self.class.maximum(maximum);
    }

    pub fn required(&mut self, required: bool) {
        self.class.required(required)
    }

    pub fn minimum(&mut self, minimum: usize) {
        self.class.minimum(minimum)
    }

    pub fn range(&mut self, minimum: usize, maximum: usize) {
        self.class.range(minimum, maximum)
    }

    pub fn name_match<S: AsRef<str>>(&self, name: S) -> bool {
        self.name.as_str() == name.as_ref()
    }

    pub fn generate_instance(&self) -> ArgumentClass {
        self.class.generate_instance()
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_help(&self) -> &str {
        match &self.help {
            Some(help) => help,
            None => "",
        }
    }

    pub fn get_maximum(&self) -> usize {
        self.class.get_maximum()
    }

    pub fn get_minimum(&self) -> usize {
        self.class.get_minimum()
    }

    pub fn get_variable_name(&self) -> &str {
        &self.variable
    }
}
