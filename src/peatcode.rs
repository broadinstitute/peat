use crate::expression::Declaration;
use crate::version::Version;

pub struct PeatCode {
    pub(crate) version: Version,
    pub(crate) declarations: Vec<Declaration>,
    pub(crate) body: String
}