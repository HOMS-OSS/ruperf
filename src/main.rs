//! #  Sample driver for perf-rust tool
//! <p> Usage: <em> perf-rust [COMMAND] [OPTION] </em>
//! where COMMAND is one of: </p>
//!<ul>
//! <li>test</li>
//! <li>stat</li>
//! </ul>

mod event;
mod gui;
mod stat;
mod test;
mod utils;

use event::*;
use gui::*;
use stat::*;
use test::*;
extern crate structopt;

use structopt::StructOpt;

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
<<<<<<< HEAD
        about = "Launches gui"
    )]
    Gui(TestOptions),
=======
        about = "Runs the gui",
    )]
    Gui(StatOptions),
>>>>>>> move: modified file structure for gui
}

fn main() {
    let opt = Opt::from_args();
    match opt {
        Opt::Stat(x) => run_stat(&x),
        Opt::Test(x) => run_test(&x),
<<<<<<< HEAD
        Opt::Gui(x) => {
            run_gui().unwrap();
        }
=======
        Opt::Gui(x) => run_gui().unwrap(),
>>>>>>> move: modified file structure for gui
    }
}
