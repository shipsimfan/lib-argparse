use crate::{ActionType, ArgumentParseError};

pub struct PositionalArgument<T> {
    name: String,
    help: Option<String>,
    minimum: usize,
    maximum: usize,
    action: ActionType<T>,
}

impl<T> PositionalArgument<T> {
    pub fn new<S: AsRef<str>>(name: S, action: ActionType<T>) -> Self {
        PositionalArgument {
            name: name.as_ref().to_owned(),
            help: None,
            minimum: 1,
            maximum: 1,
            action,
        }
    }

    pub fn help<S: AsRef<str>>(&mut self, message: S) {
        self.help = Some(message.as_ref().to_owned());
    }

    pub fn count(&mut self, new_count: usize) {
        self.maximum(new_count);
    }

    pub fn maximum(&mut self, maximum: usize) {
        if maximum > 0 && maximum < self.minimum {
            panic!("Cannot set maximum lower than minimum");
        }

        self.maximum = maximum;
    }

    pub fn required(&mut self, required: bool) {
        if required && self.minimum == 0 {
            self.minimum = 1;
        } else if !required {
            self.minimum = 0;
        }
    }

    pub fn minimum(&mut self, minimum: usize) {
        if minimum != 0 && self.maximum != 0 && self.maximum < minimum {
            panic!("Cannot set minimum hight than maximum");
        }

        self.minimum = minimum;
    }

    pub fn range(&mut self, minimum: usize, maximum: usize) {
        if maximum != 0 && maximum < minimum {
            panic!("Cannot set maximum lower than minimum");
        }

        self.maximum = maximum;
        self.minimum = minimum;
    }

    pub fn name_match<S: AsRef<str>>(&self, name: S) -> bool {
        self.name.as_str() == name.as_ref()
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
        self.maximum
    }

    pub fn get_minimum(&self) -> usize {
        self.minimum
    }

    pub fn parse(&self, arg: String, options: &mut T) -> Result<(), ArgumentParseError> {
        Ok((self.action)(vec![arg], options)?)
    }
}
