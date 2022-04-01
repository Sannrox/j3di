mod edit;
mod file;
use crate::edit::edit_json;
use crate::file::file_actions;
use anyhow::Result;
use clap::{ArgEnum, Parser, Subcommand};
use log::debug;
use serde_json::Value;
#[allow(unused)]
use std::collections::HashMap;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,

    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

#[derive(Subcommand)]
enum Commands {
    Edit {
        path: std::path::PathBuf,

        #[clap(short, long)]
        update: Option<String>,
        #[clap(short, long)]
        value: Option<String>,
        #[clap(short, long)]
        append: Option<String>,
        #[clap(short,long,arg_enum, default_value_t = Types::String)]
        type_of: Types,

        #[clap(long)]
        pretty: bool,
    },
    Search {
        // path: std::path::PathBuf,

    // #[clap(long)]
    // update: Option<String>,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum Types {
    String,
    Array,
    Number,
    Object,
    Null,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .init();

    match &cli.command {
        Commands::Edit {
            path,
            update,
            value,
            type_of,
            append,
            pretty,
        } => {
            let mut readed_data = file_actions::read_json_file(path);

            debug!(
                "Content of {}: \n {:?}",
                path.display(),
                serde_json::to_string_pretty(&readed_data).unwrap()
            );

            if let Some(edit_update) = update {
                if let Some(edit_value) = value {
                    edit_json::update(
                        &mut readed_data,
                        edit_update.to_string(),
                        edit_value.to_string(),
                        *type_of,
                    );
                } else if *type_of == Types::Null {
                    edit_json::update(
                        &mut readed_data,
                        edit_update.to_string(),
                        "".to_string(),
                        Types::Null,
                    )
                } else {
                    panic!("Value is not set - no changes")
                }
                // } else if let Some(edit_append) = append {
                //     if let Some(edit_value) = value {
                //         // let new_value: String = edit_value.to_string();
                //         // let json_path: Vec<&str> = edit_append.split('.').collect();
                //         // // recursive_json_tree(json_path, &mut readed_data, &new_value, *type_of);
                //         // write_json_file(&readed_data, path, *pretty);
                //     }
                // }
            } else {
                panic!("Please use an opiton for editing")
            }

            file_actions::write_json_file(&readed_data, path, *pretty);
            debug!(
                "Content of {} now: \n {}",
                path.display(),
                serde_json::to_string_pretty(&readed_data).unwrap()
            );
        }
        Commands::Search {} => {}
    }

    Ok(())
}

fn append_value<T: serde_json::value::Index>(
    vector: Vec<T>,
    object: &mut serde_json::Value,
    wert: &str,
) {
    let last = &vector[0];
    let new_value = Value::String(wert.to_string());
    object[last] = new_value;
}
