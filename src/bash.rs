use std::fs::File;
use std::io::Write;
use crate::util::error::Error;
use std::process::Command;
use std::path::Path;

pub(crate) fn run_bash_script(script_path: &Path, string: &String) -> Result<(), Error> {
    let mut file = File::create(script_path)?;
    file.write_all(string.as_ref())?;
    let mut cmd = Command::new("bash");
    let cmd_with_arg = cmd.arg(script_path);
    let mut child = cmd_with_arg.spawn()?;
    let status = child.wait()?;
    if !status.success() {
        let status_code =
            status.code()
                .ok_or_else(|| Error::from("Process failed and no exit code available."))?;
        Err(Error::from(format!("Process failed with exit code {}.", status_code)))
    } else {
        Ok(())
    }
}