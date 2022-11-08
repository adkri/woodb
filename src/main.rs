use clap::{command, Args, Parser, Subcommand, ValueEnum};
use rocksdb::{Options, DB};

const DB_PREFIX: &str = "localdb_";

#[derive(Debug, Parser)]
#[command(name = "woodb")]
#[command(about = "A cli db")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Get { db: String, key: String },
    Put {
        db: String,
        key: String,
        value: String,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Get { db, key } => {
            let path = DB_PREFIX.to_string() + &db;
            let db = DB::open_default(&path).unwrap();
            match db.get(&key.as_bytes()) {
                Ok(Some(value)) => println!("value: {}", String::from_utf8(value).unwrap()),
                Ok(None) => println!("value not found"),
                Err(e) => eprintln!("encountered error: {e}"),
            }
        }
        Commands::Put { db, key, value } => {
            let path = DB_PREFIX.to_string() + &db;
            let db = DB::open_default(&path).unwrap();
            match db.put(key.as_bytes(), value.as_bytes()) {
                Ok(_) => println!("added {key}={value} to db"),
                Err(e) => eprintln!("encountered error: {e}"),
            }
        }
    }
}
