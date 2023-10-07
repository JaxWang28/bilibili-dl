/***************************************************************************
*
*     Project                 ____  ____  ____  _     
*                            | __ )| __ )|  _ \| |    
*                            |  _ \|  _ \| | | | |    
*                            | |_) | |_) | |_| | |___ 
*                            |____/|____/|____/|_____|
*
*
*     A commandline tool to download bilibili video.
***************************************************************************/



use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author="jackson", version="0.0.1", about="A commandline tool to download bilibili video.", long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
    /// download video
    Download {

    },
    /// login 
    Login {

    },
}

fn main() {
    let cli = Cli::parse();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Test { list }) => {
            if *list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
        Some(Commands::Download { }) => {
            println!("Printing testing lists...");
            println!("Not printing testing lists...");
        }
        Some(Commands::Login { }) => {
            println!("Printing testing lists...");
            println!("Not printing testing lists...");
        }
        
        None => {}
    }

    // Continued program logic goes here...
}


/*
fn main() {

    print_help();
}


fn print_help() {
    println!("Usage:\tbbdl [command] [options] <URL> [pxxx]");
    println!("");
    println!("options:");
    println!("  -v, --version        print program version");
    println!("  -h, --help           give this help list");
    println!("  --qrcode             login by QR Code");
    println!("  --smscode            login by SMS Code");
    println!("  --password           login by password");
    println!("");
    println!("commands:");
    println!("  login                login in account");
    println!("  download             download,default");
}
*/
