mod movable;
mod positional;

pub use movable::MovableArgument;
pub use positional::PositionalArgument;

pub enum Argument<'a> {
    Movable(&'a mut MovableArgument),
    Positional(&'a mut PositionalArgument),
}

impl<'a> Argument<'a> {
    pub fn name<S: AsRef<str>>(&mut self, name: S) -> &mut Self {
        match self {
            Argument::Movable(movable) => movable.name(name),
            Argument::Positional(_) => {
                panic!("Cannot set more than one name on a positional argument!")
            }
        }

        self
    }

    pub fn names<S: AsRef<str>>(&mut self, names: &[S]) -> &mut Self {
        match self {
            Argument::Movable(movable) => movable.names(names),
            Argument::Positional(_) => {
                panic!("Cannot set more than one name on a positional argument!")
            }
        }

        self
    }

    pub fn hint<S: AsRef<str>>(&mut self, hint: S) -> &mut Self {
        match self {
            Argument::Movable(movable) => movable.hint(hint),
            Argument::Positional(_) => panic!("Unable to set hint on positional argument"),
        }

        self
    }

    pub fn help<S: AsRef<str>>(&mut self, message: S) -> &mut Self {
        match self {
            Argument::Movable(movable) => movable.help(message),
            Argument::Positional(positional) => positional.help(message),
        }

        self
    }

    pub fn variable_name<S: AsRef<str>>(&mut self, name: S) -> &mut Self {
        match self {
            Argument::Movable(movable) => movable.variable_name(name),
            Argument::Positional(positional) => positional.variable_name(name),
        }

        self
    }

    pub fn class<S: AsRef<str>>(&mut self, class: S) -> &mut Self {
        match self {
            Argument::Movable(movable) => movable.class(class),
            Argument::Positional(positional) => positional.class(class),
        }

        self
    }

    pub fn count(&mut self, new_count: usize) -> &mut Self {
        match self {
            Argument::Movable(movable) => movable.count(new_count),
            Argument::Positional(positional) => positional.count(new_count),
        }

        self
    }

    pub fn minimum(&mut self, minimum: usize) -> &mut Self {
        match self {
            Argument::Movable(movable) => movable.count(minimum),
            Argument::Positional(positional) => positional.minimum(minimum),
        }

        self
    }

    pub fn maximum(&mut self, maximum: usize) -> &mut Self {
        match self {
            Argument::Movable(movable) => movable.count(maximum),
            Argument::Positional(positional) => positional.maximum(maximum),
        }

        self
    }

    pub fn range(&mut self, minimum: usize, maximum: usize) -> &mut Self {
        match self {
            Argument::Movable(_) => panic!("Unable to set a range for an movable argument"),
            Argument::Positional(positional) => positional.range(minimum, maximum),
        }

        self
    }

    pub fn required(&mut self, required: bool) -> &mut Self {
        match self {
            Argument::Movable(movable) => movable.required(required),
            Argument::Positional(positional) => positional.required(required),
        }

        self
    }

    pub fn multiple(&mut self, multiple: bool) -> &mut Self {
        match self {
            Argument::Movable(movable) => movable.multiple(multiple),
            Argument::Positional(_) => match multiple {
                true => panic!("Cannot have multiple of one positional argument!"),
                false => {}
            },
        }

        self
    }
}
