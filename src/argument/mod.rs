mod optional;
mod positional;

pub use optional::OptionalArgument;
pub use positional::PositionalArgument;

pub enum Argument<'a> {
    Optional(&'a mut OptionalArgument),
    Positional(&'a mut PositionalArgument),
}

impl<'a> Argument<'a> {
    pub fn name<S: AsRef<str>>(&mut self, name: S) -> &mut Self {
        match self {
            Argument::Optional(optional) => optional.name(name),
            Argument::Positional(_) => {
                panic!("Cannot set more than one name on a positional argument!")
            }
        }

        self
    }

    pub fn names<S: AsRef<str>>(&mut self, names: &[S]) -> &mut Self {
        match self {
            Argument::Optional(optional) => optional.names(names),
            Argument::Positional(_) => {
                panic!("Cannot set more than one name on a positional argument!")
            }
        }

        self
    }

    pub fn hint<S: AsRef<str>>(&mut self, hint: S) -> &mut Self {
        match self {
            Argument::Optional(optional) => optional.hint(hint),
            Argument::Positional(_) => panic!("Unable to set hint on positional argument"),
        }

        self
    }

    pub fn help<S: AsRef<str>>(&mut self, message: S) -> &mut Self {
        match self {
            Argument::Optional(optional) => optional.help(message),
            Argument::Positional(positional) => positional.help(message),
        }

        self
    }

    pub fn variable_name<S: AsRef<str>>(&mut self, name: S) -> &mut Self {
        match self {
            Argument::Optional(optional) => optional.variable_name(name),
            Argument::Positional(positional) => positional.variable_name(name),
        }

        self
    }

    pub fn class<S: AsRef<str>>(&mut self, class: S) -> &mut Self {
        match self {
            Argument::Optional(optional) => optional.class(class),
            Argument::Positional(positional) => positional.class(class),
        }

        self
    }

    pub fn count(&mut self, new_count: usize) -> &mut Self {
        match self {
            Argument::Optional(optional) => optional.count(new_count),
            Argument::Positional(positional) => positional.count(new_count),
        }

        self
    }

    pub fn minimum(&mut self, minimum: usize) -> &mut Self {
        match self {
            Argument::Optional(optional) => optional.count(minimum),
            Argument::Positional(positional) => positional.minimum(minimum),
        }

        self
    }

    pub fn maximum(&mut self, maximum: usize) -> &mut Self {
        match self {
            Argument::Optional(optional) => optional.count(maximum),
            Argument::Positional(positional) => positional.maximum(maximum),
        }

        self
    }

    pub fn range(&mut self, minimum: usize, maximum: usize) -> &mut Self {
        match self {
            Argument::Optional(_) => panic!("Unable to set a range for an optional argument"),
            Argument::Positional(positional) => positional.range(minimum, maximum),
        }

        self
    }

    pub fn required(&mut self, required: bool) -> &mut Self {
        match self {
            Argument::Optional(_) => match required {
                true => {}
                false => panic!("Cannot set an optional argument to be not required"),
            },
            Argument::Positional(positional) => positional.required(required),
        }

        self
    }
}
