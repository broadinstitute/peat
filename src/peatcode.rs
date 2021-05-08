use declaration::Declaration;
use version::Version;

pub mod version;
pub mod value;
pub mod types;
pub mod expression;
pub mod declaration;
pub mod parse;
pub mod evaluate;
pub mod tokenize;
mod tree;

pub struct PeatCode {
    pub(crate) version: Version,
    pub(crate) declarations: Vec<Declaration>,
    pub(crate) body: String
}
