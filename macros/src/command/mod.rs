use input::Input;
use output::Output;
use proc_macro_util::{ast::DeriveItem, Result};

mod input;
mod output;

pub use output::{DefaultValue, Description, FlagHelpOutput, FlagHelpUsageOutput, FlagInfo};

pub fn generate(item: DeriveItem) -> Result<Output> {
    Ok(Input::extract(item)?.into_output())
}
