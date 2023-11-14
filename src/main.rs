use tokio::sync::oneshot;
use tokio::sync::mpsc;

use clap::{Parser, Subcommand};
use tokio::task::JoinSet;
use std::sync::{Arc, Mutex};

use reqwest::Client;




mod object_parser;
use object_parser::init_object_parser;
use object_parser::Object;
mod resource_selector;
use resource_selector::init_res_selector;
mod downloader;
use downloader::init_downloader;
mod multimedia_processor;
use multimedia_processor::init_multimedia_processor;



#[derive(Subcommand)]
enum Commands {
    /* TODO */
    /// login your account
    login { 

    },

    /* TODO */
    /// just show video info
    showinfo {

    }
}

#[derive(Parser)]
#[command(author="jackson", version="0.0.1", about="A commandline program to download bilibili video.", long_about = None)]
struct Cli {
    /* TODO: change to set */
    /* TODO: should change the url to others */
    object: Vec<String>,

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
    command: Option<Commands>,
}




#[tokio::main]
async fn main(){
    let cli = Cli::parse();
     match &cli.command {
        Some(Commands::login{ /*list */}) => {

        }
        Some(Commands::showinfo{ /*list */}) => {

        }
        None => {
            //let url = cli.url;
            download(cli.object).await;

        }
    }
}



/* default download command. */
async fn download(object: Vec<String>/* */) {

    /* create client */
    let client = Arc::new(Mutex::new(Client::new()));
    /* TODO: init client */


    /* create channel. */
    let (tx_object_parser, rx_res_selector) = mpsc::channel::<i32>(16);
    /*
    let (tx_res_selector, rx_downloader) = oneshot::channel::<i32>();
    let (tx_downloader, rx_media_processor) = oneshot::channel::<i32>();
    */
    

    /* TODO: init */
    /* url parser should start after others ?*/
    let object_parser = init_object_parser(&client, Object::Url(object),0, 0, tx_object_parser);
    let res_selector = init_res_selector(rx_res_selector);
    let downloader = init_downloader();
    let multimedia_processor = init_multimedia_processor();

    let mut set = JoinSet::new();

    /* start */
    set.spawn(object_parser);
    set.spawn(res_selector);
    set.spawn(downloader);
    set.spawn(multimedia_processor);

    while let Some(_) = set.join_next().await {}
}
