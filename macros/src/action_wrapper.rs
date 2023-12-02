use proc_macro_util::ast::Expression;

pub enum ActionWrapper<'a, 'b> {
    Wrap(&'b Expression<'a>),
    NoWrap(&'b Expression<'a>),
}
