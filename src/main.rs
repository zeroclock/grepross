extern crate grep_into_table;

use log::*;
use pretty_env_logger;
use structopt::{clap, StructOpt};
use std::fs::File;
use std::io::prelude::*;
use anyhow::Context as _;

use grep_into_table::generate_csv;

#[derive(StructOpt, Debug)]
#[structopt(name = "grep_into_table")]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
struct Opt {
    #[structopt(name = "filename")]
    filename: String,
}

fn main() -> anyhow::Result<()> {
    // initialize
    pretty_env_logger::init();

    // parsing options
    let opt = Opt::from_args();
    debug!("opt: {:?}", opt);

    // opening file
    let mut f = File::open(&opt.filename)
        .with_context(|| format!("Could not open file: {}", &opt.filename))?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)
     .with_context(|| format!("Something went wrong reading file: {}", &opt.filename))?;

    // generate csv
    generate_csv(&contents);

    Ok(())
}
