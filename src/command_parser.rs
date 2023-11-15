/*
 *
 *
 *
 *
 *
 *
 *
 *
 * */


/* */
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author="jackson", version="0.0.1", about="A commandline program to download bilibili video.", long_about = None)]
pub struct Cli {
    /* TODO: change to set */
    /* TODO: should change the url to others */
    pub object: Vec<String>,

    /* TODO: args */

    /* v
     * a
     * d
     * s
     * c */
    /*
    #[arg(short, long)]
    choose:String,
    */


    #[command(subcommand)]
    pub command: Option<Commands>,
}


#[derive(Subcommand)]
pub enum Commands {
    /* TODO */
    /// login your account
    login { 

    },

    /* TODO */
    /// just show video info
    showinfo {

    }
}

use crate::data_model::StartConfig;
pub fn parse_command() -> (Vec<String>, StartConfig){
    todo!();
    let cli = Cli::parse();
    /*
     match &cli.command {
        Some(Commands::login{ /*list */}) => {
            //login().await;

        }
        Some(Commands::showinfo{ /*list */}) => {

        }
        None => {
            //let url = cli.url;
            //download(cli.object).await;
            todo!();
        }
    }
*/
}

