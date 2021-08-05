//! # Main driver.
//! <p> Usage: <em> ruperf [COMMAND] [OPTION] </em>
//! where COMMAND is one of: </p>
//! <ul>
//! <li>test</li>
//! <li>stat</li>
//! <li>gui</li>
//! </ul>

mod bindings;
mod event;
mod gui;
mod stat;
mod test;
mod utils;

extern crate structopt;
use event::*;
use gui::*;
use stat::*;
use structopt::StructOpt;
use test::*;

/// Define command line options.
#[derive(Debug, StructOpt)]
enum Opt {
    #[structopt(
        setting = structopt::clap::AppSettings::TrailingVarArg,
        setting = structopt::clap::AppSettings::AllowLeadingHyphen,
        name = "stat",
        about = "Collects hardware/software event counters",
    )]
    Stat(StatOptions),
    #[structopt(
        setting = structopt::clap::AppSettings::TrailingVarArg,
        setting = structopt::clap::AppSettings::AllowLeadingHyphen,
        name = "test",
        about = "Runs sanity tests"
    )]
    Test(TestOptions),
    #[structopt(
        setting = structopt::clap::AppSettings::TrailingVarArg,
        setting = structopt::clap::AppSettings::AllowLeadingHyphen,
        name = "gui",
        about = "Launches gui"
    )]
    Gui(GuiOptions),
}

fn main() {
    let opt = Opt::from_args();
    match opt {
        Opt::Stat(x) => run_stat(x),
        Opt::Test(x) => run_test(&x),
        Opt::Gui(x) => {
            run_gui(&x).unwrap();
        }
    }
}
