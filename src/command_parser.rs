use clap::{Parser, Subcommand};
use serde::Deserialize;
use serde::Serialize;
use std::io::{self, Write};
use url::Url;
use tokio::{task, sync::mpsc};

/* myself */
use crate::data_model;
use crate::target_parser::Target;
use crate::target_parser::Video;

#[derive(Parser)]
#[command(author="jackson", version="0.0.1", about="A commandline program to download bilibili video.", long_about = None)]
pub struct Cli {
    /* TODO: change to set */
    /* TODO: should change the url to others */
    pub targets: Vec<String>,

    /* TODO: args */

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
    Login { 

    },

    /* TODO */
    /// just show video info
    ShowInfo {

    }
}




pub struct CommandParser{
    sender: mpsc::Sender<Target>,

}


impl CommandParser {
    pub fn new (sender: mpsc::Sender<Target>) -> CommandParser{
        CommandParser {
            sender,
        }
    }
    pub async fn start(self) {
        let cli = Cli::parse();
        match &cli.command {
            Some(Commands::Login{ /*list */}) => {
                login().await;
            }
            Some(Commands::ShowInfo{ /*list */}) => {
                todo!();
            }
            None => {
                download(self.sender, cli.targets).await;
                //Some(cli.targets);
            }

        }
    }
}
















use crate::target_parser::TargetParser;
use crate::resource_selector::ResourceSelector;
use crate::downloader::Downloader;
use crate::multimedia_processor::MultimediaProcessor;



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
    let response: data_model::Response<Data>= response.json().await.unwrap();
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



async fn download(sender: mpsc::Sender<Target>, targets: Vec<String> /* others */) {
    for target in targets {
        task::spawn(preproc_target(sender.clone(), target));
    }
}

async fn preproc_target(sender: mpsc::Sender<Target>, target: String /* others */){
    /* TODO: */
    sender.send(Target::VIDEO(
            Video {
                aid: None,
                bvid: Some(target), 
                cid: None,
                flags: None,
                title: None,
                page_range: (1, 1),
                page_id: None,

            })).await.unwrap();
}



