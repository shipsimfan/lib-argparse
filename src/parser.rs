use crate::{
    argument::{MovableArgument, PositionalArgument},
    argument_class::ArgumentClass,
    Argument, ArgumentParseError, Arguments,
};
use std::collections::HashMap;

pub struct ArgumentParser {
    program_name: Option<String>,
    version: Option<String>,
    description: Option<String>,
    epilogue: Option<String>,
    usage: Option<String>,
    help: bool,
    movable_arguments: Vec<MovableArgument>,
    positional_arguments: Vec<PositionalArgument>,
}

impl ArgumentParser {
    pub fn new() -> Self {
        ArgumentParser {
            program_name: None,
            version: None,
            description: None,
            epilogue: None,
            usage: None,
            help: true,
            movable_arguments: Vec::new(),
            positional_arguments: Vec::new(),
        }
    }

    pub fn program_name<S: AsRef<str>>(&mut self, name: S) -> &mut Self {
        self.program_name = Some(name.as_ref().to_owned());
        self
    }

    pub fn version<S: AsRef<str>>(&mut self, version: S) -> &mut Self {
        self.version = Some(version.as_ref().to_owned());
        self
    }

    pub fn description<S: AsRef<str>>(&mut self, description: S) -> &mut Self {
        self.description = Some(description.as_ref().to_owned());
        self
    }

    pub fn epilogue<S: AsRef<str>>(&mut self, epilogue: S) -> &mut Self {
        self.epilogue = Some(epilogue.as_ref().to_owned());
        self
    }

    pub fn usage<S: AsRef<str>>(&mut self, usage: S) -> &mut Self {
        self.usage = Some(usage.as_ref().to_owned());
        self
    }

    pub fn help(&mut self, enable: bool) -> &mut Self {
        self.help = enable;
        self
    }

    pub fn add_argument<S: AsRef<str>>(&mut self, name: S) -> Argument {
        if name.as_ref().starts_with('-') {
            let index = self.movable_arguments.len();
            self.movable_arguments.push(MovableArgument::new(name));
            Argument::Movable(self.movable_arguments.get_mut(index).unwrap())
        } else {
            let index = self.positional_arguments.len();
            self.positional_arguments
                .push(PositionalArgument::new(name));
            Argument::Positional(self.positional_arguments.get_mut(index).unwrap())
        }
    }

    pub fn parse_args_env(&self) -> Result<Arguments, ArgumentParseError> {
        let mut args = std::env::args();
        self.parse_args(&mut args)
    }

    pub fn parse_args<I: Iterator<Item = String>>(
        &self,
        args: &mut I,
    ) -> Result<Arguments, ArgumentParseError> {
        // Skip the first argument and get the program name if nescessary
        let program_name = match &self.program_name {
            Some(name) => {
                args.next();
                name.to_owned()
            }
            None => match args.next() {
                Some(name) => name,
                None => String::new(),
            },
        };

        // Parse the arguments
        let mut variables: HashMap<String, ArgumentClass> = HashMap::new();
        let mut positional_index = 0;
        let mut current_positional =
            self.positional_arguments
                .get(positional_index)
                .map(|positional| {
                    (
                        positional.generate_instance(),
                        positional.get_maximum(),
                        positional.get_variable_name(),
                    )
                });
        let mut count = 0;

        'main: while let Some(arg) = args.next() {
            if arg.starts_with('-') {
                if arg == "--version" {
                    match &self.version {
                        Some(version) => print_version(&program_name, version),
                        None => {}
                    }
                }

                if arg == "-h" || arg == "--help" {
                    match self.help {
                        true => self.print_help(&program_name),
                        false => {}
                    }
                }

                for argument in &self.movable_arguments {
                    if argument.name_match(&arg) {
                        let (name, output) = argument.parse(&arg, args)?;

                        if argument.is_multiple() {
                            match variables.get_mut(&name) {
                                Some(variable) => {
                                    variable.extend(output);

                                    continue 'main;
                                }
                                None => {}
                            }

                            variables.insert(name, output);

                            continue 'main;
                        } else {
                            match variables.insert(name, output) {
                                Some(_) => return Err(ArgumentParseError::MultipleDefinition(arg)),
                                None => continue 'main,
                            }
                        }
                    }
                }

                return Err(ArgumentParseError::UnknownArgument(arg));
            } else {
                current_positional = match current_positional {
                    Some((mut positional, max, variable_name)) => {
                        positional.insert(arg)?;
                        if max != 0 {
                            count += 1;

                            if count >= max {
                                positional_index += 1;
                                match variables.insert(variable_name.to_owned(), positional) {
                                    Some(_) => {
                                        return Err(ArgumentParseError::MultipleDefinition(
                                            variable_name.to_owned(),
                                        ))
                                    }
                                    None => {}
                                }

                                self.positional_arguments
                                    .get(positional_index)
                                    .map(|positional| {
                                        (
                                            positional.generate_instance(),
                                            positional.get_maximum(),
                                            positional.get_variable_name(),
                                        )
                                    })
                            } else {
                                Some((positional, max, variable_name))
                            }
                        } else {
                            Some((positional, max, variable_name))
                        }
                    }
                    None => return Err(ArgumentParseError::UnexpectedArgument(arg)),
                }
            }
        }

        match current_positional {
            Some((positional, _, variable_name)) => {
                match variables.insert(variable_name.to_owned(), positional) {
                    Some(_) => {
                        return Err(ArgumentParseError::MultipleDefinition(
                            variable_name.to_owned(),
                        ))
                    }
                    None => {}
                }
            }
            None => {}
        }

        // Verify positional minimums
        for argument in &self.positional_arguments {
            let minimum = argument.get_minimum();
            if minimum != 0 {
                match variables.get(argument.get_variable_name()) {
                    Some(variable) => {
                        if variable.len() < minimum {
                            return Err(ArgumentParseError::TooFewArguments(
                                argument.get_name().to_owned(),
                                minimum,
                                variable.len(),
                            ));
                        }
                    }
                    None => {
                        return Err(ArgumentParseError::MissingRequiredArgument(
                            argument.get_name().to_owned(),
                        ))
                    }
                }
            }
        }

        // Verify required movables
        for movable in &self.movable_arguments {
            if movable.is_required() {
                match variables.get(movable.get_variable_name()) {
                    Some(_) => {}
                    None => {
                        return Err(ArgumentParseError::MissingRequiredArgument(
                            movable.get_names()[0].to_owned(),
                        ))
                    }
                }
            }
        }

        Ok(crate::arguments::new(variables))
    }

    fn generate_usage(&self) -> String {
        let mut string = format!("%(prog) ");

        for movable in &self.movable_arguments {
            string.push_str(&format!("[{}] ", movable.generate_usage()));
        }

        for positional in &self.positional_arguments {
            if positional.get_minimum() == 0 {
                string.push('[');
            }

            string.push_str(&format!("{}", positional.get_name()));

            if positional.get_maximum() == 0 {
                string.push_str(" ...");
            }

            if positional.get_minimum() == 0 {
                string.push(']');
            }

            if positional.get_maximum() == 0 {
                break;
            }

            string.push(' ');
        }

        string
    }

    fn display_help_positionals(&self, program_name: &str) {
        if self.positional_arguments.len() == 0 {
            return;
        }

        let mut positional_length = 8;
        for positional in &self.positional_arguments {
            let len = positional.get_name().len() + 2;
            if len > positional_length {
                positional_length = len;
            }
        }

        println!("\nPOSITIONAL ARGUMENTS:");
        for positional in &self.positional_arguments {
            let len = positional_length - positional.get_name().len();

            for _ in 0..len {
                print!(" ");
            }

            println!(
                "{}    {}",
                positional.get_name(),
                positional.get_help().replace("%(prog)", program_name)
            );
        }
    }

    fn display_help_movables(&self, program_name: &str) {
        if self.movable_arguments.len() == 0 {
            return;
        }

        let mut longest_line = if self.version.is_some() { 9 } else { 6 };

        for movable in &self.movable_arguments {
            let names = movable.get_names();

            let offset = if names[0].len() == 2 { 1 } else { 0 };

            let mut len = 0;
            for i in offset..names.len() {
                len += names[i].len() + 2;
            }

            if len > 0 {
                len -= 2;
            }

            if movable.has_hint() {
                len += movable.generate_hint().len() + 1;
            }

            if longest_line < len {
                longest_line = len;
            }
        }

        println!("\nMOVABLE ARGUMENTS:");
        for movable in &self.movable_arguments {
            print!("  ");

            let names = movable.get_names();

            let (start, extra) = if names[0].len() == 2 {
                print!("{}", names[0]);
                if names.len() > 1 {
                    print!(",");
                    (1, false)
                } else {
                    (1, true)
                }
            } else {
                print!("   ");
                (0, true)
            };

            print!(" ");

            let mut printed_length = 0;
            for i in start..names.len() {
                print!("{}", names[i]);
                printed_length += names[i].len();

                if i < names.len() - 1 {
                    print!(", ");
                }
            }

            if movable.has_hint() {
                let hint = movable.generate_hint();
                print!("{}", hint);
                printed_length += hint.len();
            }

            for _ in printed_length..longest_line {
                print!(" ");
            }

            if extra {
                print!(" ");
            }

            println!(
                "    {}",
                movable.get_help().replace("%(prog)", program_name)
            );
        }

        print!("  -h, --help");
        for _ in 0..longest_line - 6 {
            print!(" ");
        }
        println!("    Display this help message");

        if self.version.is_some() {
            print!("      --version");
            for _ in 0..longest_line - 9 {
                print!(" ");
            }
            println!("    Display the version");
        }
    }

    fn print_help(&self, program_name: &str) -> ! {
        match &self.description {
            Some(description) => println!("{}\n", description.replace("%(prog)", program_name)),
            None => {}
        }

        println!(
            "USAGE:\n    {}",
            match &self.usage {
                Some(usage) => usage.to_owned(),
                None => self.generate_usage(),
            }
            .replace("%(prog)", program_name)
        );

        self.display_help_positionals(program_name);
        self.display_help_movables(program_name);

        match &self.epilogue {
            Some(epilogue) => println!("\n{}", epilogue.replace("%(prog)", program_name)),
            None => {}
        }

        std::process::exit(0);
    }
}

fn print_version(program_name: &str, version: &str) -> ! {
    println!("{}", version.replace("%(prog)", program_name));
    std::process::exit(0);
}
