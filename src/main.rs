// Copyright Rivtower Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use clap::Clap;
use log::info;

use git_version::git_version;
const GIT_VERSION: &str = git_version!(args = ["--tags", "--always", "--dirty=-modified"], fallback = "unknown");

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Clap)]
#[clap(version = "0.1.0", author = "Rivtower Technologies.")]
struct Opts {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(short = "c", long = "config", default_value = "default.conf")]
    config: String,
    /// Some input. Because this isn't an Option<T> it's required to be used
    input: String,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    /// print version information from git
    #[clap(name = "gversion")]
    GitVersion,
    /// A help message for the Test subcommand
    #[clap(name = "test", version = "1.3", author = "Someone Else")]
    Test(Test),
}

/// A subcommand for controlling testing
#[derive(Clap)]
struct Test {
    /// Print debug info
    #[clap(short = "d")]
    debug: bool,
}

fn main() {
    ::std::env::set_var("RUST_BACKTRACE", "full");

    log4rs::init_file("template-log4rs.yaml", Default::default()).unwrap();
    for _ in 0..10000u64 {
        info!("Starting...");
    }

    let opts: Opts = Opts::parse();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    println!("Value for config: {}", opts.config);
    println!("Using input file: {}", opts.input);

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    match opts.subcmd {
        SubCommand::GitVersion => {
            println!("git version: {}", GIT_VERSION);
            std::process::exit(0);
        },
        SubCommand::Test(t) => {
            if t.debug {
                println!("Printing debug info...");
            } else {
                println!("Printing normally...");
            }
        }
    }

    // more program logic goes here...
    println!("...");
}
