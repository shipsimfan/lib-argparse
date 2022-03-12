use crate::{
    argument::{MovableArgument, PositionalArgument},
    ActionType, Argument, ArgumentParseError,
};

pub enum DashAction<T> {
    Reject,
    Ignore,
    Positional,
    Accept(ActionType<T>),
    Collect(ActionType<T>),
}

pub struct ArgumentParser<T = ()> {
    program_name: Option<String>,
    version: Option<String>,
    description: Option<String>,
    epilogue: Option<String>,
    usage: Option<String>,
    help: bool,
    movable_arguments: Vec<MovableArgument<T>>,
    movable_header: Option<String>,
    positional_arguments: Vec<PositionalArgument<T>>,
    positional_header: Option<String>,
    dash_action: DashAction<T>,
    dash_dash_action: DashAction<T>,
}

impl<T> ArgumentParser<T> {
    pub fn new() -> Self {
        ArgumentParser {
            program_name: None,
            version: None,
            description: None,
            epilogue: None,
            usage: None,
            help: true,
            movable_arguments: Vec::new(),
            movable_header: None,
            positional_arguments: Vec::new(),
            positional_header: None,
            dash_action: DashAction::Reject,
            dash_dash_action: DashAction::Reject,
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

    pub fn movable_header<S: AsRef<str>>(&mut self, header: S) -> &mut Self {
        self.movable_header = Some(header.as_ref().to_owned());
        self
    }

    pub fn positional_header<S: AsRef<str>>(&mut self, header: S) -> &mut Self {
        self.positional_header = Some(header.as_ref().to_owned());
        self
    }

    pub fn add_argument<S: AsRef<str>>(&mut self, name: S, action: ActionType<T>) -> Argument<T> {
        if name.as_ref().starts_with('-') {
            let index = self.movable_arguments.len();
            self.movable_arguments
                .push(MovableArgument::new(name, action));
            Argument::Movable(self.movable_arguments.get_mut(index).unwrap())
        } else {
            let index = self.positional_arguments.len();
            self.positional_arguments
                .push(PositionalArgument::new(name, action));
            Argument::Positional(self.positional_arguments.get_mut(index).unwrap())
        }
    }

    pub fn dash_action(&mut self, action: DashAction<T>) {
        self.dash_action = action;
    }

    pub fn dash_dash_action(&mut self, action: DashAction<T>) {
        self.dash_dash_action = action;
    }

    pub fn parse_args_env(&self, options: T) -> Result<T, ArgumentParseError> {
        let mut args = std::env::args();
        self.parse_args(&mut args, options)
    }

    pub fn parse_args<I: Iterator<Item = String>>(
        &self,
        args: &mut I,
        mut options: T,
    ) -> Result<T, ArgumentParseError> {
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
        let mut positional_index = 0;
        let mut positional_counts = vec![0];
        let mut current_maximum = self
            .positional_arguments
            .get(positional_index)
            .map(|positional| positional.get_maximum());

        let mut required_movables = Vec::new();
        for argument in &self.movable_arguments {
            if argument.is_required() {
                required_movables.push(argument);
            }
        }

        'main: while let Some(arg) = args.next() {
            if arg == "-" {
                match self.dash_action {
                    DashAction::Reject => return Err(ArgumentParseError::UnknownArgument(arg)),
                    DashAction::Ignore => continue 'main,
                    DashAction::Positional => {}
                    DashAction::Accept(action) => {
                        (action)(&[], &mut options)?;
                        continue 'main;
                    }
                    DashAction::Collect(action) => {
                        let mut values = Vec::new();
                        while let Some(arg) = args.next() {
                            values.push(arg);
                        }

                        (action)(values.as_slice(), &mut options)?;
                        continue 'main;
                    }
                }
            } else if arg == "--" {
                match self.dash_dash_action {
                    DashAction::Reject => return Err(ArgumentParseError::UnknownArgument(arg)),
                    DashAction::Ignore => continue 'main,
                    DashAction::Positional => {}
                    DashAction::Accept(action) => {
                        (action)(&[], &mut options)?;
                        continue 'main;
                    }
                    DashAction::Collect(action) => {
                        let mut values = Vec::new();
                        while let Some(arg) = args.next() {
                            values.push(arg);
                        }

                        (action)(values.as_slice(), &mut options)?;
                        continue 'main;
                    }
                }
            } else if arg.starts_with('-') {
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
                        argument.parse(args, &mut options)?;
                        continue 'main;
                    }
                }

                return Err(ArgumentParseError::UnknownArgument(arg));
            }

            current_maximum = match current_maximum {
                Some(maximum) => {
                    self.positional_arguments[positional_index].parse(arg, &mut options)?;
                    positional_counts[positional_index] += 1;
                    if maximum != 0 && positional_counts[positional_index] >= maximum {
                        positional_counts.push(0);
                        positional_index += 1;

                        self.positional_arguments
                            .get(positional_index)
                            .map(|positional| (positional.get_maximum()))
                    } else {
                        Some(maximum)
                    }
                }
                None => return Err(ArgumentParseError::UnexpectedArgument(arg)),
            }
        }

        // Verify positional minimums
        for i in 0..self.positional_arguments.len() {
            let argument = &self.positional_arguments[i];
            let minimum = argument.get_minimum();
            if positional_counts[i] < minimum {
                if positional_counts[i] > 0 {
                    return Err(ArgumentParseError::TooFewArguments(
                        argument.get_name().to_owned(),
                        minimum,
                        positional_counts[i],
                    ));
                } else {
                    return Err(ArgumentParseError::MissingRequiredArgument(
                        argument.get_name().to_owned(),
                    ));
                }
            }
        }

        // Verify required movables
        match required_movables.pop() {
            Some(movable) => Err(ArgumentParseError::MissingRequiredArgument(
                movable.get_names()[0].to_owned(),
            )),
            None => Ok(options),
        }
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

        println!(
            "\n{}",
            match &self.positional_header {
                Some(header) => header,
                None => "POSITIONAL ARGUMENTS:",
            }
        );
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

        let mut longest_line = if self.version.is_some() { 13 } else { 10 };

        for movable in &self.movable_arguments {
            let names = movable.get_names();

            let offset = if names[0].len() == 2 { 1 } else { 0 };

            let mut len = 4;
            for i in offset..names.len() {
                len += names[i].len() + 2;
            }

            if len > 4 {
                len -= 2;
            }

            if movable.has_hint() {
                len += movable.generate_hint().len() + 1;
            }

            if longest_line < len {
                longest_line = len;
            }
        }

        println!(
            "\n{}",
            match &self.movable_header {
                Some(header) => header,
                None => "MOVABLE ARGUMENTS:",
            }
        );
        for movable in &self.movable_arguments {
            print!("  ");

            let names = movable.get_names();

            let (start, mut printed_length) = if names[0].len() == 2 {
                print!("{}", names[0]);
                if names.len() > 1 {
                    print!(", ");
                    (1, 4)
                } else {
                    (1, 2)
                }
            } else {
                print!("    ");
                (0, 4)
            };

            for i in start..names.len() {
                print!("{}", names[i]);
                printed_length += names[i].len();

                if i < names.len() - 1 {
                    print!(", ");
                    printed_length += 2;
                }
            }

            if movable.has_hint() {
                let hint = movable.generate_hint();
                print!(" {}", hint);
                printed_length += hint.len() + 1;
            }

            for _ in printed_length..longest_line {
                print!(" ");
            }

            println!(
                "    {}",
                movable.get_help().replace("%(prog)", program_name)
            );
        }

        print!("  -h, --help");
        for _ in 0..longest_line - 10 {
            print!(" ");
        }
        println!("    Display this help message");

        if self.version.is_some() {
            print!("      --version");
            for _ in 0..longest_line - 13 {
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
