use crate::types::Bindings;
use crate::error::Error;

mod delims {
    pub(crate) const PRE: &str = "<:";
    pub(crate) const POST: &str = ":>";
}

pub(crate) fn substitute(body: &String, bindings: &Bindings) -> Result<String, Error> {
    let mut body_new = body.clone();
    for (id, value) in bindings {
        let pattern = format!("{}{}{}", delims::PRE, id, delims::POST);
        let substitute = format!("{}", value);
        body_new = body_new.replace(pattern.as_str(), substitute.as_str());
    }
    Ok(body_new)
}