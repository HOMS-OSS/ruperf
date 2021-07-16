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
<<<<<<< HEAD
        name = "gui",
        about = "Runs the gui",
    )]
    Gui(StatOptions),
=======
        name = "test",
        about = "Runs sanity tests"
    )]
    Test(TestOptions),
>>>>>>> 50499bd4c3b5631fe63001da4c41f9ad267fa161
}

fn main() {
    let opt = Opt::from_args();
    match opt {
        Opt::Stat(x) => run_stat(&x),
<<<<<<< HEAD
        Opt::Gui(x) => {
            run_gui().unwrap();
        }
=======
        Opt::Test(x) => run_test(&x),
>>>>>>> 50499bd4c3b5631fe63001da4c41f9ad267fa161
    }
}
