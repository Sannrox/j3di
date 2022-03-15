use anyhow::{Context, Error, Result};
use clap::{ArgEnum, Parser, Subcommand};
use log::debug;
use serde_json::Value;
use std::any::type_name;
#[allow(unused)]
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

// fn type_of<T>(_: T) -> &'static str {
//     type_name::<T>()
// }
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
enum Types {
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
            let mut readed_data = read_json_file(path);

            debug!(
                "Content of {}: \n {:?}",
                path.display(),
                serde_json::to_string_pretty(&readed_data).unwrap()
            );
            if let Some(edit_update) = update {
                if let Some(edit_value) = value {
                    let new_value: Vec<String> =
                        edit_value.split(',').map(|s| s.to_string()).collect();
                    let json_path: Vec<String> =
                        edit_update.split('.').map(|s| s.to_string()).collect();
                    recursive_json_tree(json_path, &mut readed_data, &new_value, *type_of);
                    write_json_file(&readed_data, path, *pretty);
                    debug!(
                        "Content of {} now: \n {}",
                        path.display(),
                        serde_json::to_string_pretty(&readed_data).unwrap()
                    );
                } else {
                    panic!("Value is not set - no changes")
                }
            } else if let Some(edit_append) = append {
                if let Some(edit_value) = value {
                    let new_value: String = edit_value.to_string();
                    let json_path: Vec<&str> = edit_append.split('.').collect();
                    // recursive_json_tree(json_path, &mut readed_data, &new_value, *type_of);
                    // write_json_file(&readed_data, path, *pretty);
                }
            } else {
                panic!("Please use an opiton for editing")
            }
        }
        Commands::Search {} => {}
    }

    Ok(())
}

fn write_json_file(data: &Value, path: &Path, pretty: bool) {
    let output: String = if pretty {
        serde_json::to_string_pretty(&data)
            .with_context(|| format!("could not parse json: \n `{}`", &data.to_string()))
            .unwrap()
    } else {
        serde_json::to_string(&data)
            .with_context(|| format!("could not parse json: \n `{}`", &data.to_string()))
            .unwrap()
    };

    let mut file = File::create(path)
        .with_context(|| format!("could not read file `{}`", &path.display()))
        .unwrap();

    file.write_all(output.as_bytes())
        .expect("Could not write to file");
}

fn read_json_file(path: &Path) -> Value {
    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("could not read file `{}`", &path.display()))
        .unwrap();

    serde_json::from_str(&content)
        .with_context(|| format!("could not parse json: \n `{}`", &content.to_string()))
        .unwrap()
}

fn recursive_json_tree<'a, T: serde_json::value::Index>(
    mut vector: Vec<T>,
    object: &'a mut serde_json::Value,
    wert: &'a Vec<String>,
    type_of: Types,
    // writer: &dyn Fn(Vec<T>, &mut serde_json::Value, &str),
) where
    T: std::clone::Clone + std::fmt::Debug,
{
    if vector.len() > 1 {
        let first_ele = vector[0].clone();
        vector.remove(0);

        recursive_json_tree(vector.to_vec(), &mut object[first_ele], wert, type_of)
    } else {
        match type_of {
            Types::String => {
                if wert.len() == 1 {
                    let thing: String = wert.first().unwrap().to_string();
                    update_value_to_string(vector, object, &thing)
                }
            }
            Types::Array => update_value_to_array(
                vector,
                object,
                wert.iter()
                    .map(|s| serde_json::to_value(s).unwrap())
                    .collect(),
            ),
            Types::Null => {}
            Types::Number => {}
            Types::Object => {}
        }
    }
}

fn update_value_to_string<T: serde_json::value::Index>(
    vector: Vec<T>,
    object: &mut serde_json::Value,
    wert: &String,
) {
    let last = &vector[0];
    let new_value = Value::String(wert.to_string());
    object[last] = new_value;
}

#[test]
fn upadte_string() {
    let vector = vec!["name"];
    let wert = String::from("Jane Doe");

    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    let ref_data = r#"
        {
            "name": "Jane Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    let mut v: Value = serde_json::from_str(data).unwrap();
    let ref_v: Value = serde_json::from_str(ref_data).unwrap();

    update_value_to_string(vector, &mut v, &wert);

    assert_eq!(v, ref_v)
}

fn update_value_to_array<T: serde_json::value::Index>(
    vector: Vec<T>,
    object: &mut serde_json::Value,
    wert: Vec<Value>,
) {
    let last = &vector[0];
    let new_value = Value::Array(wert);
    object[last] = new_value;
}

#[test]
fn upadte_array() {
    let vector = vec!["name"];
    let array = r#"["Jane Doe", "Francis Doe", "Matt Doe"]"#;

    let test_array: Vec<Value> = serde_json::from_str(array).unwrap();

    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    let ref_data = r#"
        {
            "name": ["Jane Doe", "Francis Doe", "Matt Doe"],
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    let mut v: Value = serde_json::from_str(data).unwrap();
    let ref_v: Value = serde_json::from_str(ref_data).unwrap();

    update_value_to_array(vector, &mut v, test_array);

    assert_eq!(v, ref_v)
}

fn update_value_to_object<T: serde_json::value::Index>(
    vector: Vec<T>,
    object: &mut serde_json::Value,
    wert: serde_json::Map<String, Value>,
) {
    let last = &vector[0];
    let new_value = Value::Object(wert);
    object[last] = new_value;
}

#[test]
fn upadte_object() {
    let vector = vec!["name"];
    let array = r#"{ "Forename": "Jane", "Surname": "Doe"}"#;

    let test_array: serde_json::Map<String, Value> = serde_json::from_str(array).unwrap();

    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    let ref_data = r#"
        {
            "name":{ "Forename": "Jane", "Surname": "Doe"},
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    let mut v: Value = serde_json::from_str(data).unwrap();
    let ref_v: Value = serde_json::from_str(ref_data).unwrap();

    update_value_to_object(vector, &mut v, test_array);

    assert_eq!(v, ref_v)
}

fn update_value_to_number<T: serde_json::value::Index>(
    vector: Vec<T>,
    object: &mut serde_json::Value,
    wert: serde_json::Number,
) {
    let last = &vector[0];
    let new_value = Value::Number(wert);
    object[last] = new_value;
}

#[test]
fn upadte_number() {
    let vector = vec!["age"];
    let number = r#"15"#;

    let test_number: serde_json::Number = serde_json::from_str(number).unwrap();

    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    let ref_data = r#"
        {
            "name": "John Doe",
            "age": 15,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    let mut v: Value = serde_json::from_str(data).unwrap();
    let ref_v: Value = serde_json::from_str(ref_data).unwrap();

    update_value_to_number(vector, &mut v, test_number);

    assert_eq!(v, ref_v)
}

fn update_value_to_null<T: serde_json::value::Index>(
    vector: Vec<T>,
    object: &mut serde_json::Value,
) {
    let last = &vector[0];
    let new_value = Value::Null;
    object[last] = new_value;
}

#[test]
fn upadte_null() {
    let vector = vec!["phones"];

    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    let ref_data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": null
        }"#;

    let mut v: Value = serde_json::from_str(data).unwrap();
    let ref_v: Value = serde_json::from_str(ref_data).unwrap();

    update_value_to_null(vector, &mut v);

    assert_eq!(v, ref_v)
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
// #[test]
// fn read_json_from_string() {
//     let test_string: String = r#"{ 'Test':'test' }"#.to_string();
//     let result = read_json(&test_string).unwrap();
//     assert_eq!(result, "{Test: test}")

// }
