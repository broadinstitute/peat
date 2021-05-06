use std::fs::File;
use std::io::Write;
use crate::error::Error;
use std::process::Command;

pub(crate) fn run_bash_script(string: String) -> Result<(), Error> {
    let file_name = "script.sh";
    let mut file = File::create(file_name)?;
    file.write_all(string.as_ref())?;
    let mut cmd = Command::new("bash");
    let cmd_with_arg = cmd.arg(file_name);
    let mut child = cmd_with_arg.spawn()?;
    let status = child.wait()?;
    if !status.success() {
        let status_code =
            status.code().ok_or(Error::from("Process failed and no exit code available."))?;
        Err(Error::from(format!("Process failed with exit code {}.", status_code)))
    } else {
        Ok(())
    }
}