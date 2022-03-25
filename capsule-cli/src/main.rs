use std::fmt::format;

// Copyright 2022 the original author or authors.
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
use clap::{Parser, Subcommand};

use capsule::{CommandError, create_application};

#[derive(Parser)]
#[clap(name = "capsule")]
#[clap(about = "CLI to interact with Capsule", long_about = None)]
#[clap(version = "1.0")]
struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// create application
    create,
    /// manage applications
    apps,
    /// manage capsules
    ps,
}

fn main() {
    let args: Cli = Cli::parse();

    let result = match &args.command {
        Commands::create => create_application::handle(),
        _ => Err(CommandError { message: format!(" is ") }),
    };

    if let Err(CommandError { message }) = result {
        println!("{}", message)
    }
}
