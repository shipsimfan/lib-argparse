use proc_macro::Delimiter;
use proc_macro_util::{
    tokens::{Colon, Dot, DoubleColon, Equals, Identifier, Literal, SemiColon},
    Generator, Parse, ToTokens,
};

pub struct Parser {
    variable_name: Identifier,
    equals: Equals,
    name: Literal,
    description: Option<Literal>,
}

fn generate_parser_type_name(generator: &mut Generator) {
    DoubleColon::default().to_tokens(generator);
    generator.identifier_string("argparse");
    DoubleColon::default().to_tokens(generator);
    generator.identifier_string("Parser");
}

impl Parse for Parser {
    fn parse(parser: &mut proc_macro_util::Parser) -> proc_macro_util::Result<Self> {
        let variable_name = parser
            .parse()
            .map_err(|error| error.append("expected a variable name for the parser"))?;

        let equals = parser.parse()?;

        let name = parser
            .parse()
            .map_err(|error| error.append("expected a name"))?;
        let description = parser
            .parse()
            .map_err(|error| error.append("expected a description or the end"))?;

        Ok(Parser {
            variable_name,
            equals,
            name,
            description,
        })
    }
}

impl ToTokens for Parser {
    fn to_tokens(&self, generator: &mut proc_macro_util::Generator) {
        generator.identifier_string("const");
        generator.identifier(self.variable_name.clone());
        Colon::default().to_tokens(generator);
        generate_parser_type_name(generator);

        self.equals.to_tokens(generator);

        generate_parser_type_name(generator);
        DoubleColon::default().to_tokens(generator);
        generator.identifier_string("new");
        generator.group(Delimiter::Parenthesis);

        Dot::default().to_tokens(generator);
        generator.identifier_string("name");

        let mut name_parameters = generator.group(Delimiter::Parenthesis);
        name_parameters.literal(self.name.clone());

        if let Some(description) = self.description.clone() {
            Dot::default().to_tokens(generator);
            generator.identifier_string("description");

            let mut description_parameters = generator.group(Delimiter::Parenthesis);
            description_parameters.literal(description);
        }

        SemiColon::default().to_tokens(generator);
    }
}
