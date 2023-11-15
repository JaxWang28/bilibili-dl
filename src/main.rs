/***************************************************************************
*                                                                          *
*     Project                 ____  ____  ____  _                          *
*                            | __ )| __ )|  _ \| |                         *
*                            |  _ \|  _ \| | | | |                         *
*                            | |_) | |_) | |_| | |___                      *
*                            |____/|____/|____/|_____|                     *
*                                                                          *
*                                                                          *
*     A commandline tool to download bilibili video.                       *
***************************************************************************/
use tokio::sync::mpsc;
use tokio::task::JoinSet;
use reqwest::Client;
use reqwest;
use lazy_static::lazy_static;
mod target_parser;
mod resource_selector;
mod downloader;
mod multimedia_processor;
mod command_parser;
use command_parser::parse_command;
mod data_model;
use crate::data_model::Undefined;
use crate::target_parser::TargetParser;
use crate::resource_selector::ResourceSelector;
use crate::downloader::Downloader;
use crate::multimedia_processor::MultimediaProcessor;



lazy_static! {
    static ref GLOBAL_CLIENT: Client = {
        let cookie_store = {
            /* TODO: cookie path */
            if let Ok(file) = std::fs::File::open("cookies.json").map(std::io::BufReader::new)
            {
              // use re-exported version of `CookieStore` for crate compatibility
                reqwest_cookie_store::CookieStore::load_json(file).unwrap()
            }
            else
            {
                reqwest_cookie_store::CookieStore::new(None)
            }
        };
        let cookie_store = reqwest_cookie_store::CookieStoreMutex::new(cookie_store);
        let cookie_store = std::sync::Arc::new(cookie_store);
        // Build a `reqwest` Client, providing the deserialized store
        reqwest::Client::builder()
        .cookie_provider(std::sync::Arc::clone(&cookie_store))
        .build()
        .unwrap()
    };
}




#[tokio::main]
async fn main(){
    let target_parser = TargetParser::new();
    let resource_selector = ResourceSelector::new();
    let downloader = Downloader::new();
    let multimedia_processor = MultimediaProcessor::new();

    if let Some(targets) = parse_command(&target_parser, &resource_selector, &downloader,  &multimedia_processor).await{
        /* TODO: channel create and set */
       
        let (_tx, _rx) = mpsc::channel::<Undefined>(16);

        /* start */
        let mut set = JoinSet::new();
        set.spawn(target_parser.start(targets));
        set.spawn(resource_selector.start());
        set.spawn(downloader.start());
        set.spawn(multimedia_processor.start());
        while let Some(_) = set.join_next().await {}
        println!("main() finished");
    }
}



