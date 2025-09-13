use crate::prelude::*;

#[derive(Parser, Clone, Debug)]
pub struct Opt {
    /// Define a glob pattern for the flattening of the files.
    #[clap(value_parser)]
    pub input: String,

    /// Define the way of handling duplicate file names due to the flattening process.
    #[clap(long, value_enum, default_value_t = DuplicateAction::Skip)]
    pub on_duplicate: DuplicateAction,

    /// The format for handling duplicate file names (only applicable for [`DuplicateAction::Rename`])
    /// Variables: {stem} - file stem / {n} - count / {ext} - File extension
    #[clap(long, default_value = "{stem} - ({n}){ext}", requires("on_duplicate"))]
    pub duplicate_format: String,

    /// Run the application withoiut making any changes.
    #[clap(long, action=ArgAction::SetTrue)]
    pub dry_run: bool,
}

#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub enum DuplicateAction {
    Abort,
    Skip,
    Rename,
}
