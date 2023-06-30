use clap::{Command, Arg, crate_version, crate_authors, crate_name, crate_description};

struct Database {
    host: String,
    port: String,
    database: String,
    schema: String,
    user: String,
    password: String,
}

/// PostgreSQL data schema and dictionary documenter using the mdBook format.
///
/// This tool will the following PostgreSQL objects:
/// + Tables
/// + Views
/// + Functions
/// + Stored Procedures
/// + Triggers
/// + Indexes
/// + Sequences
/// + Types
/// + Schemas
/// + Defaults
/// + Constraints
/// + Check Constraints
/// + Exclusion Constraints
/// + Foreign Key Constraints
/// + Primary Key Constraints
/// + Unique Constraints
/// + Triggers
/// + Comments
///
/// The CLI will accept the following options:
/// + Database host server
/// + Database port
/// + Database name
/// + Database user
/// + Database password
/// + Output directory
///
fn main() {
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
                .required(true)
                .default_value("localhost")
        )
        .arg(
            Arg::new("port")
                .short('P')
                .long("port")
                .value_name("PORT")
                .help("Database port")
                .action(clap::ArgAction::Set)
                .required(false)
                .default_value("5432")
        )
        .arg(
            Arg::new("database")
                .short('D')
                .long("database")
                .value_name("DATABASE")
                .help("Database name")
                .action(clap::ArgAction::Set)
                .required(true)
        )
        .arg(
            Arg::new("schema")
                .short('S')
                .long("schema")
                .value_name("SCHEMA")
                .help("Database schema (ex. public)")
                .action(clap::ArgAction::Set)
                .required(false)
                .default_value("public")
        )
        .arg(
            Arg::new("user")
                .short('U')
                .long("user")
                .value_name("USER")
                .help("Database user")
                .action(clap::ArgAction::Set)
                .required(true)
        )
        .arg(
            Arg::new("password")
                .short('W')
                .long("password")
                .value_name("PASSWORD")
                .help("Database password")
                .action(clap::ArgAction::Set)
                .required(true)
        )
        .arg(
            Arg::new("output_path")
                .short('O')
                .long("output")
                .value_name("OUTPUT")
                .help("Output path")
                .action(clap::ArgAction::Set)
                .required(false)
                .default_value("./output")
        )
        .get_matches();

    println!("host: {:?}", matches.get_one::<String>("host"));
    println!("port: {:?}", matches.get_one::<String>("port"));
    println!("database: {:?}", matches.get_one::<String>("database"));
    println!("schema: {:?}", matches.get_one::<String>("schema"));
    println!("user: {:?}", matches.get_one::<String>("user"));
    println!("password: {:?}", matches.get_one::<String>("password"));
    println!("output: {:?}", matches.get_one::<String>("output_path"));
}
