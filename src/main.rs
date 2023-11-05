use std::{
    collections::HashMap,
    fs,
    process::{self, Command, ExitCode, ExitStatus},
};

use clap::{Parser, Subcommand};
use log::*;

mod paramparsing;
mod pass;
mod templating;
mod verbosity;

#[derive(Parser)]
#[command(author, version)]
#[command(propagate_version = true)]
#[command(about = "A simple git credential helper for gnu pass", long_about = None)]
struct Cli {
    #[arg(long, short = 't')]
    ///The path to the template file (You can use template syntax in this path)
    template: String,

    #[arg(long, short = 'p')]
    ///The password name ex: 'www/github.com/main' (You can use template syntax in here)
    password: String,

    #[command(flatten)]
    verbosity: verbosity::Verbosity,

    #[command(subcommand)]
    operation: Commands,
}

#[derive(Subcommand)]
enum Commands {
    ///Stores the credentials in the backing helper
    Store,
    ///Deletes the credentials from the backing helper
    Erase,
    ///Gets the stored credentials
    Get,
}

fn main() {
    let cli = Cli::parse();
    stderrlog::new()
        .module(module_path!())
        .verbosity(cli.verbosity.log_level())
        .init()
        .unwrap();

    let params = paramparsing::parse_from_stdin();

    debug!("params={:?}", &params);

    let template_path = templating::populate(&cli.template, &params);
    debug!("template_path={}", template_path);

    let template = fs::read_to_string(&template_path).unwrap_or_else(|err| {
        error!(
            "FATAL: Could not read template at '{}'\n{}",
            template_path, err
        );
        process::exit(-1);
    });

    match cli.operation {
        Commands::Get => {
            let output = HashMap::new();
            let pass_output = pass::get_password(&cli.password);

            paramparsing::write_to_stdout(output);
        }
        Commands::Store => {
            let template_resolved = templating::populate(&template, &params);

            pass::insert_password(&cli.password, &template_resolved);
        }
        Commands::Erase => {}
    }
}
