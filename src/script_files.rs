use std::path::PathBuf;
use crate::util::error::Error;

pub(crate) struct ScriptNameGenerator {
    dir: PathBuf,
    counter: u32,
}

impl ScriptNameGenerator {
    pub(crate) fn from_temp_dir<'a>() -> Result<ScriptNameGenerator, Error> {
        Ok(ScriptNameGenerator::new(tempfile::tempdir()?.into_path()))
    }
    pub(crate) fn new<'a>(dir: PathBuf) -> ScriptNameGenerator {
        let counter = 0u32;
        ScriptNameGenerator { dir, counter }
    }

    pub(crate) fn next(&mut self) -> PathBuf {
        let mut script_path_buf = self.dir.clone();
        script_path_buf.push(format!("script{}.sh", self.counter));
        self.counter += 1;
        script_path_buf
    }
}

impl Drop for ScriptNameGenerator {
    fn drop(&mut self) {
        match std::fs::remove_dir_all(&self.dir) {
            Ok(_) => {}
            Err(error) => {
                println!("Failed to remove directory {}: {}",
                         self.dir.to_str().unwrap_or("<unprintable>"), error)
            }
        }
    }
}