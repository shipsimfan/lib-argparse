use proc_macro::Delimiter;
use proc_macro_util::{
    tokens::{Dot, DoubleColon, Literal},
    Parse, ToTokens,
};

pub struct Parser {
    name: Literal,
    description: Option<Literal>,
}

impl Parse for Parser {
    fn parse(parser: &mut proc_macro_util::Parser) -> proc_macro_util::Result<Self> {
        let name = parser
            .parse()
            .map_err(|error| error.append("expected a name"))?;
        let description = parser
            .parse()
            .map_err(|error| error.append("expected a description or the end"))?;

        Ok(Parser { name, description })
    }
}

impl ToTokens for Parser {
    fn to_tokens(&self, generator: &mut proc_macro_util::Generator) {
        let double_colon = DoubleColon::default();

        double_colon.to_tokens(generator);
        generator.identifier_string("argparse");
        double_colon.to_tokens(generator);
        generator.identifier_string("Parser");
        double_colon.to_tokens(generator);
        generator.identifier_string("new");

        let mut parameters = generator.group(Delimiter::Parenthesis);
        parameters.literal(self.name.clone());

        if let Some(description) = self.description.clone() {
            Dot::default().to_tokens(generator);
            generator.identifier_string("description");

            let mut parameters = generator.group(Delimiter::Parenthesis);
            parameters.literal(description);
        }
    }
}
