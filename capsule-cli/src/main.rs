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

use capsule::{cmd_apps, cmd_create_application, cmd_ps, CommandError};

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
    Create,
    /// manage applications
    Apps,
    /// manage capsules
    Ps,
}

fn main() {
    let args: Cli = Cli::parse();

    // let result = match &args.command {
    //     Commands::Create => cmd_create_application::handle(),
    //     Commands::Apps => cmd_apps::handle(),
    //     Commands::Ps => cmd_ps::handle(),
    // };
    //
    // if let Err(CommandError { message }) = result {
    //     println!("{}", message)
    // }
}
