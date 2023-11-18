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
use command_parser::CommandParser;
mod data_model;
use crate::data_model::Undefined;
use crate::target_parser::TargetParser;
use crate::resource_selector::ResourceSelector;
use crate::downloader::Downloader;
use crate::multimedia_processor::MultimediaProcessor;
use crate::target_parser::Target;
use crate::target_parser::Video;



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




use std::time::Instant;
#[tokio::main]
async fn main(){
    let start = Instant::now();
    let (c2t_tx, tfc_rx) = mpsc::channel::<Target>(16);
    let command_parser = CommandParser::new(c2t_tx);

    let (t2r_tx, rft_rx) = mpsc::channel::<Video>(16);
    let target_parser = TargetParser::new(&GLOBAL_CLIENT, tfc_rx, t2r_tx);
    let (r2d_tx, dfr_rx) = mpsc::channel::<(reqwest::Response, reqwest::Response, String)>(16);
    let resource_selector = ResourceSelector::new(&GLOBAL_CLIENT, rft_rx, r2d_tx);
    let (d2m_tx, mfd_rx) = mpsc::channel::<String>(16);
    let downloader = Downloader::new(&GLOBAL_CLIENT, dfr_rx, d2m_tx);
    
    let multimedia_processor = MultimediaProcessor::new(mfd_rx);


    /* start */
    let mut set = JoinSet::new();
    set.spawn(command_parser.start());
    set.spawn(target_parser.start());
    set.spawn(resource_selector.start());
    set.spawn(downloader.start());
    set.spawn(multimedia_processor.start());
    while let Some(_) = set.join_next().await {}
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}



