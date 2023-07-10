//! PostgreSQL data schema and dictionary documenter using the mdBook format.
//!
//! This tool will the following PostgreSQL objects:
//! + Tables
//! + Views
//! + Functions
//! + Stored Procedures
//! + Triggers
//! + Indexes
//! + Sequences
//! + Types
//! + Schemas
//! + Defaults
//! + Constraints
//! + Check Constraints
//! + Exclusion Constraints
//! + Foreign Key Constraints
//! + Primary Key Constraints
//! + Unique Constraints
//! + Triggers
//! + Comments
//!
//! The CLI will accept the following options:
//! + Database host server
//! + Database port
//! + Database name
//! + Database user
//! + Database password
//! + Output directory

mod database;

use clap::{crate_authors, crate_description, crate_name, crate_version, Arg, Command};
use std::error::Error;

fn main() {
    if let Err(e) = get_args().and_then(|config| run(&config)) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}

#[derive(Debug)]
pub struct Config {
    host: String,
    port: String,
    database: String,
    schema: String,
    user: String,
    password: String,
    output_path: String,
}

pub fn run(config: &Config) -> Result<database::Database, Box<dyn Error>> {
    dbg!(config);
    let db = database::run(config);

    db
}

pub fn get_args() -> Result<Config, Box<dyn Error>> {
    let matches = Command::new("pgbook")
        .version(crate_version!())
        .name(crate_name!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::new("host")
                .short('H')
                .long("host")
                .value_name("HOST")
                .help("Database host server")
                .action(clap::ArgAction::Set)
                //.required(true)
                .default_value("localhost"),
        )
        .arg(
            Arg::new("port")
                .short('P')
                .long("port")
                .value_name("PORT")
                .help("Database port")
                .action(clap::ArgAction::Set)
                .required(false)
                .default_value("5432"),
        )
        .arg(
            Arg::new("database")
                .short('D')
                .long("database")
                .value_name("DATABASE")
                .help("Database name")
                .action(clap::ArgAction::Set)
                //.required(true)
                .default_value("postgres"),
        )
        .arg(
            Arg::new("schema")
                .short('S')
                .long("schema")
                .value_name("SCHEMA")
                .help("Database schema (ex. public)")
                .action(clap::ArgAction::Set)
                .required(false)
                .default_value("public"),
        )
        .arg(
            Arg::new("user")
                .short('U')
                .long("user")
                .value_name("USER")
                .help("Database user")
                .action(clap::ArgAction::Set)
                //.required(true)
                .default_value("username"),
        )
        .arg(
            Arg::new("password")
                .short('W')
                .long("password")
                .value_name("PASSWORD")
                .help("Database password")
                .action(clap::ArgAction::Set)
                //.required(true)
                .default_value("password"),
        )
        .arg(
            Arg::new("output_path")
                .short('O')
                .long("output")
                .value_name("OUTPUT")
                .help("Output path")
                .action(clap::ArgAction::Set)
                .required(false)
                .default_value("./output"),
        )
        .get_matches();

    Ok(Config {
        host: matches
            .get_one::<String>("host")
            .expect("host is required")
            .to_string(),
        port: matches
            .get_one::<String>("port")
            .expect("port is required")
            .to_string(),
        database: matches
            .get_one::<String>("database")
            .expect("database is required")
            .to_string(),
        schema: matches
            .get_one::<String>("schema")
            .expect("schema is required")
            .to_string(),
        user: matches
            .get_one::<String>("user")
            .expect("user is required")
            .to_string(),
        password: matches
            .get_one::<String>("password")
            .expect("password is required")
            .to_string(),
        output_path: matches
            .get_one::<String>("output_path")
            .expect("output_path is required")
            .to_string(),
    })
}
