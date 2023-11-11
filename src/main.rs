use std::fs;

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
    ///The path to the template file (You can use template syntax here)
    template: String,

    #[arg(long, short = 'p')]
    ///The password name ex: 'www/github.com/main' (You can use template syntax in here)
    pass_name: String,

    #[command(flatten)]
    verbosity: verbosity::Verbosity,

    #[command(subcommand)]
    operation: Commands,
}

#[derive(Subcommand)]
enum Commands {
    ///Store the credentials in the password store
    Store,
    ///Delete the credentials from the password store
    Erase,
    ///Gets the credentials stored in the password store
    Get,
}

macro_rules! die {
    ($($arg:tt)+) => {log::error!("FATAL: {}", format!($($arg)+)); std::process::exit(-1);}
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

    let template_path = templating::populate(
        &templating::parse(&cli.template).unwrap_or_else(|err| {
            die!("Failed to parse template '{}'\n{}", &cli.template, err);
        }),
        &params,
    );
    debug!("template_path={}", template_path);

    let template = fs::read_to_string(&template_path).unwrap_or_else(|err| {
        die!("Could not read template at '{}'\n{}", template_path, err);
    });
    let template = templating::parse(&template).unwrap_or_else(|err| {
        die!("Failed to parse template at '{}'\n{}", template_path, err);
    });

    let pass_name = templating::populate(
        &templating::parse(&cli.pass_name).unwrap_or_else(|err| {
            die!("Could not parse template '{}'\n{}", &cli.template, err);
        }),
        &params,
    );

    match cli.operation {
        Commands::Get => {
            let pass_output = pass::get_password(&pass_name);
            //unwrap here because the template is already validated when parsing
            let output = templating::get_params(&template, &pass_output).unwrap();

            paramparsing::write_to_stdout(output);
        }
        Commands::Store => {
            let template_resolved = templating::populate(&template, &params);

            pass::insert_password(&pass_name, &template_resolved);
        }
        Commands::Erase => {
            pass::remove_password(&pass_name);
        }
    }
}
