use crate::{ActionType, ArgumentParseError};

pub struct MovableArgument<T> {
    names: Vec<String>,
    hint: Option<String>,
    help: Option<String>,
    required: bool,
    count: usize,
    action: ActionType<T>,
}

impl<T> MovableArgument<T> {
    pub fn new<S: AsRef<str>>(name: S, action: ActionType<T>) -> Self {
        MovableArgument {
            names: vec![name.as_ref().to_owned()],
            hint: None,
            help: None,
            required: false,
            count: 0,
            action,
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

    pub fn hint<S: AsRef<str>>(&mut self, hint: S) {
        self.hint = Some(hint.as_ref().to_owned());
    }

    pub fn help<S: AsRef<str>>(&mut self, message: S) {
        self.help = Some(message.as_ref().to_owned());
    }

    pub fn count(&mut self, new_count: usize) {
        self.count = new_count;
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

        if self.count == 0 {
            return string;
        }

        string.push(' ');
        string.push_str(&self.generate_hint());

        string
    }

    pub fn generate_hint(&self) -> String {
        if self.count == 0 {
            String::new()
        } else {
            match &self.hint {
                Some(hint) => hint.to_owned(),
                None => match self.count > 1 {
                    true => format!("{} VALUES", self.count),
                    false => format!("VALUE"),
                },
            }
        }
    }

    pub fn has_hint(&self) -> bool {
        self.count > 0
    }

    pub fn is_required(&self) -> bool {
        self.required
    }

    pub fn get_names(&self) -> &[String] {
        self.names.as_slice()
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
        iter: &mut I,
        options: &mut T,
    ) -> Result<(), ArgumentParseError> {
        let mut values = Vec::with_capacity(self.count);

        for i in 0..self.count {
            match iter.next() {
                Some(value) => values.push(value),
                None => {
                    return Err(ArgumentParseError::TooFewArguments(
                        self.names[0].clone(),
                        self.count,
                        i,
                    ))
                }
            }
        }

        Ok((self.action)(values, options)?)
    }
}
