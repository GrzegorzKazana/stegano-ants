#![allow(dead_code)]
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "profiler")] {
        #[macro_use]
        // using extern instead of use, to import
        // flamer attribute once and do not need to conditionally
        // import it elsewhere
        extern crate flamer;

        use flame as f;
        use std::fs::File;
    }
}

use clap::Clap;

#[macro_use]
mod macros;

mod ant_colony;
mod app;
mod cli;
mod common;
mod images;
mod steganography;

use app::App;
use cli::Opts;
use common::cli_output::{CliOutput, CliOutputs};

fn main() {
    let opts: Opts = Opts::parse();
    let cli = CliOutputs::from_bool(opts.quiet);

    match App::new(opts, &cli).run() {
        Result::Err(msg) => cli.print(&format!("{}", msg)),
        Result::Ok(summary) => cli.print(&format!("{}", summary.to_string())),
    }

    cfg_if! {
        if #[cfg(feature = "profiler")] {
            let latest_file_name_html = ".profiles/_latest.html";
            let latest_file_name_json = ".profiles/_latest.json";
            f::dump_html(File::create(latest_file_name_html).unwrap()).unwrap();
            f::dump_json(&mut File::create(latest_file_name_json).unwrap()).unwrap();
        }
    }
}
