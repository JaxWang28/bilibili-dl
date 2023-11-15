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
use tokio::sync::oneshot;
use tokio::sync::mpsc;
use clap::{Parser, Subcommand};
use tokio::task::JoinSet;
use std::sync::{Arc, Mutex};
use std::io::{self, Write};
use reqwest::Client;
use reqwest;
use serde::Deserialize;
use serde::Serialize;
use url::{Url};
use lazy_static::lazy_static;

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


mod target_parser;
use target_parser::init_object_parser;
use target_parser::Object;
mod resource_selector;
use resource_selector::init_res_selector;
mod downloader;
use downloader::init_downloader;
mod multimedia_processor;
use multimedia_processor::init_multimedia_processor;
mod command_parser;
use command_parser::Cli;
use command_parser::Commands;
use command_parser::parse_command;


//static client: Client;
#[tokio::main]
async fn main(){
    let (targets, startconfig) = parse_command();
    start(targets, startconfig).await;
}



mod data_model;
use data_model::StartConfig;
async fn start(targets: Vec<String>, startconfig: StartConfig) {
    todo!();
}


/* default download command. */
async fn download(object: Vec<String>/* */) {

    /* create client */
    /* TODO: init client */


    /* create channel. */
    let (tx_object_parser, rx_res_selector) = mpsc::channel::<i32>(16);
    /*
    let (tx_res_selector, rx_downloader) = oneshot::channel::<i32>();
    let (tx_downloader, rx_media_processor) = oneshot::channel::<i32>();
    */
    

    /* TODO: init */
    /* url parser should start after others ?*/
    let object_parser = init_object_parser(&GLOBAL_CLIENT,Object::Url(object),0, 0, tx_object_parser);
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

#[derive(Deserialize, Serialize, Debug)]
pub struct Response <T>{
    code: i32,
    message: String,
    ttl: i32,
    data: T,
}

/* TODO: 优化*/
async fn login() {
    let cookie_store = reqwest_cookie_store::CookieStore::new(None);
    let cookie_store = reqwest_cookie_store::CookieStoreMutex::new(cookie_store);
    let cookie_store = std::sync::Arc::new(cookie_store);
    let client = 
        reqwest::Client::builder()
        .cookie_provider(std::sync::Arc::clone(&cookie_store))
        .build()
        .unwrap();
    
    let response = client.get("https://passport.bilibili.com/x/passport-login/web/qrcode/generate").send().await.unwrap();

    if !response.status().is_success() {
        return
    }
    #[derive(Deserialize,Serialize, Debug)]
    struct Data {
        url: String,
        qrcode_key: String,
    }
    let response:Response<Data>= response.json().await.unwrap();
    println!("{:?}", response);
    let mut input = String::new();
    loop {
        input.clear();  // Clear the previous input

        print!("Continue? (y/N): ");
        io::stdout().flush().unwrap();  // Flush to ensure the prompt is displayed before waiting for input

        io::stdin().read_line(&mut input).unwrap();

        if input.trim().eq_ignore_ascii_case("y") {
            println!("Continuing...");
            break;  // Exit the loop
        } else {
            println!("Invalid input. Please enter 'y' or 'Y'.");
        }
    }
    // 解析 URL
    let mut url = Url::parse("https://passport.bilibili.com/x/passport-login/web/qrcode/poll").expect("Failed to parse URL");

    // 添加查询参数
    url.query_pairs_mut()
        .append_pair("qrcode_key", &response.data.qrcode_key);

    //println!("{:#?}", url.to_string());
    let _response = client.get(&url.to_string()).send().await.unwrap();

    {
      // Write store back to disk
      let mut writer = std::fs::File::create("cookies.json")
          .map(std::io::BufWriter::new)
          .unwrap();
      let store = cookie_store.lock().unwrap();
      store.save_json(&mut writer).unwrap();
    }
}

