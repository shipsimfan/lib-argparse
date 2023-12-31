use proc_macro_util::{ast::Expression, to_tokens, Generator, ToTokens};

pub enum ActionWrapper<'a, 'b> {
    Wrap(&'b Expression<'a>),
    NoWrap(&'b Expression<'a>),
}

impl<'a, 'b> ToTokens for ActionWrapper<'a, 'b> {
    fn to_tokens(&self, generator: &mut Generator) {
        match self {
            ActionWrapper::Wrap(action) => {
                to_tokens! { generator
                    Ok(#action)
                }
            }
            ActionWrapper::NoWrap(action) => action.to_tokens(generator),
        }
    }
}
