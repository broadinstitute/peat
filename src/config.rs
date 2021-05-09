use crate::util::error::Error;
use std::env;
use clap::{App, Arg};

pub(crate) struct Config {
    pub(crate) parse_only: bool,
    pub(crate) dry_run: bool,
    pub(crate) input_file: Option<String>,
}

mod names {
    pub(crate) const FILE: &str = "FILE";
    pub(crate) const PARSE_ONLY: &str = "PARSE_ONLY";
    pub(crate) const DRY_RUN: &str = "DRY_RUN";
}

pub(crate) fn get_config() -> Result<Config, Error> {
    let app =
        App::new(clap::crate_name!())
            .author(clap::crate_authors!())
            .version(clap::crate_version!())
            .about(clap::crate_description!())
            .arg(Arg::with_name(names::PARSE_ONLY)
                .short("r")
                .long("parse-only")
                .takes_value(false)
                .help("Parse only. Do not evaluate expressions and do not run jobs."))
            .arg(Arg::with_name(names::DRY_RUN)
                .short("d")
                .long("dry-run")
                .takes_value(false)
                .help("Parse and evaluate expressions, but do not actually run jobs."))
            .arg(Arg::with_name(names::FILE)
                .value_name("peat file")
                .takes_value(true));
    let matches = app.get_matches_safe()?;
    let parse_only = matches.is_present(names::PARSE_ONLY);
    let dry_run = matches.is_present(names::DRY_RUN);
    let input_file =
        matches.value_of(names::FILE).map(|s| { String::from(s) });
    Ok(Config { parse_only, dry_run, input_file })
}
