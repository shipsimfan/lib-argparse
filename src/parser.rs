use crate::FlagArgument;
use std::{borrow::Cow, ops::Deref};

pub struct Parser<T, E = ()> {
    program_name: Option<Cow<'static, str>>,
    description: Option<Cow<'static, str>>,

    flag_arguments: Vec<FlagArgument<T, E>>,
}

impl<T, E> Parser<T, E> {
    pub fn new() -> Self {
        Parser {
            program_name: None,
            description: None,

            flag_arguments: Vec::new(),
        }
    }

    pub fn program_name(&self) -> Option<&str> {
        self.program_name
            .as_ref()
            .map(|long_name| long_name.deref())
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_ref().map(|long_name| long_name.deref())
    }

    pub fn flag_arguments(&self) -> &[FlagArgument<T, E>] {
        &self.flag_arguments
    }

    pub fn set_program_name<S: Into<Cow<'static, str>>>(&mut self, program_name: S) {
        self.program_name = Some(program_name.into());
    }

    pub fn set_description<S: Into<Cow<'static, str>>>(&mut self, description: S) {
        self.description = Some(description.into());
    }

    pub fn add_flag_argument(&mut self, flag_argument: FlagArgument<T, E>) {
        self.flag_arguments.push(flag_argument);
    }
}
