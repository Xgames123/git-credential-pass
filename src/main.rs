use clap::{Parser, Subcommand};
use log::*;
use std::{collections::HashMap, fs};
use templating::SyntaxTree;
use utils::*;

use crate::pass::PassError;

mod paramparsing;
mod pass;
mod templating;
mod utils;
mod verbosity;

#[cfg(test)]
mod tests;

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
#[command(propagate_version = true)]
struct Cli {
    #[arg(long, short = 't')]
    ///The path to the template file (You can use template syntax here)
    template: String,

    #[arg(long, short = 'p')]
    ///The password name ex: 'www/github.com/main' (You can use template syntax in here)
    pass_name: String,

    ///The amount of times to retry if the password request failed (Useful for if your pinentry program doesn't have retries)
    #[arg(long, short = 'r', default_value = "0")]
    retries: u32,

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

fn main() {
    let cli = Cli::parse();
    stderrlog::new()
        .module(module_path!())
        .verbosity(cli.verbosity.log_level())
        .init()
        .unwrap();

    let params = paramparsing::parse_from_stdin();

    debug!("params={:?}", &params);

    let template_path = abs_path(&templating::populate(
        &templating::parse(&cli.template).unwrap_or_else(|err| {
            die!("Failed to parse template '{}'\n{}", &cli.template, err);
        }),
        &params,
    ));
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

    let result = match cli.operation {
        Commands::Get => get(cli, &pass_name, template),
        Commands::Store => store(cli, &pass_name, template, &params),
        Commands::Erase => pass::remove_password(&pass_name, cli.retries),
    };
    result.unwrap_or_else(|err| match err {
        pass::PassError::Io(err) => {
            die!("{}", err);
        }
        pass::PassError::Non0ExitCode(code, _) => {
            error!("{}", err);
            std::process::exit(code);
        }
    });
}

fn get(cli: Cli, pass_name: &str, template: SyntaxTree) -> Result<(), PassError> {
    let pass_output = pass::get_password(&pass_name, cli.retries)?;
    let output = templating::get_params(&template, &pass_output).unwrap();

    Ok(paramparsing::write_to_stdout(output))
}

fn store(
    cli: Cli,
    pass_name: &str,
    template: SyntaxTree,
    params: &HashMap<String, String>,
) -> Result<(), PassError> {
    let template_resolved = templating::populate(&template, &params);

    pass::insert_password(&pass_name, &template_resolved, cli.retries)
}
