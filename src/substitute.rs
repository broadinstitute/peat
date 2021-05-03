use crate::types::Bindings;
use crate::error::Error;

mod delims {
    const PRE: &str = "<:";
    const POST: &str = ":>";
}

pub(crate) fn substitute(body: &String, bindings: &Bindings) -> Result<String, Error> {
    // let mut body_new = body;
    // for (id, value) in
    //
    body.replace("x", "y");
    todo!()
}