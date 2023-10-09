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
use clap::{Parser, Subcommand};
//use url::{Url, ParseError,Host, Position};
use url::{Url};
use reqwest;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use std::error::Error;



#[derive(Deserialize, Serialize, Debug)]
struct Response <T>{
    code: i32,
    message: String,
    ttl: i32,
    data: T,
}


#[derive(Subcommand)]
enum Commands {
    /// login your account
    Login { 
        //name: Option<String> 
    },
    /// download video
    Download {
        url: String

    }
}

#[derive(Parser)]
#[command(author="jackson", version="0.0.1", about="A commandline program to download bilibili video.", long_about = None)]
struct Cli {
    /*
    /// URLs to download
    url: Vec<String>,
    */
    #[command(subcommand)]
    command: Commands,
}

fn login() -> Result<(), reqwest::Error> {
    let cookie_store = reqwest_cookie_store::CookieStore::new(None);
    let cookie_store = reqwest_cookie_store::CookieStoreMutex::new(cookie_store);
    let cookie_store = std::sync::Arc::new(cookie_store);


    // Build a `reqwest` Client, providing the deserialized store
    let client = reqwest::blocking::Client::builder()
    .cookie_provider(std::sync::Arc::clone(&cookie_store))
    .build()
    .unwrap();

    let response = client.get("https://passport.bilibili.com/x/passport-login/web/qrcode/generate").send()?;



    #[derive(Deserialize,Serialize)]
    struct Data {
        url: String,
        qrcode_key: String,
    }


    let response:Response<Data>= response.json()?;
    println!("{}", response.data.url);


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
    let _response = client.get(&url.to_string()).send()?;

    {
      // Write store back to disk
      let mut writer = std::fs::File::create("cookies.json")
          .map(std::io::BufWriter::new)
          .unwrap();
      let store = cookie_store.lock().unwrap();
      store.save_json(&mut writer).unwrap();
    }
    Ok(())
}



struct video{
    bvid: String, 
    cid: i32,
}

#[derive(Deserialize,Serialize, Debug)]
struct Obj {
    id:i32,
    baseUrl:String,
    base_url:String,
}


#[derive(Deserialize,Serialize, Debug)]
struct Dash {
    video: Vec<Obj>,
    audio: Vec<Obj>,
}


fn fetchstream_core(mut response: reqwest::blocking::Response, client: &reqwest::blocking::Client, name: &str) {
    let mut size: u64 = 0;
    match response.headers().get(CONTENT_LENGTH) {
        Some(length) => {
            size = length.to_str().unwrap().parse().unwrap();
            println!("File size: {} bytes", size);
        }
        None => println!("Content-Length header is missing!"),
    }


    let mut file = OpenOptions::new().create(true).append(true).open(name).unwrap();
    
    let mut num = 0;
    let mut current_offset = file.seek(SeekFrom::Current(0)).unwrap();
    println!("Current offset: {}", current_offset);

    
    loop {
        let range = format!("bytes={}-", current_offset);
        println!("{}", range);
        let mut response = client.get(response.url().to_string()).header(RANGE, range).send().unwrap();

        let result = copy(&mut response, &mut file);
        match result {
            Ok(value) => {
                println!("ok");
                break;
            },
            Err(err) => {
                println!("error: {:#?}", err);
                // handle the error case
            },
        }
        current_offset = file.seek(SeekFrom::Current(0)).unwrap();
        println!("Current offset: {}", current_offset);
    }
}

fn fetchstream(dash:Dash) -> i32{
    let cookie_store = {
      if let Ok(file) = std::fs::File::open("cookies.json")
        .map(std::io::BufReader::new)
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
    let client = reqwest::blocking::Client::builder()
    .cookie_provider(std::sync::Arc::clone(&cookie_store))
    .build()
    .unwrap();

    let mut response_result: Option<reqwest::blocking::Response> = None;

    for video in &dash.video {
        response_result = Some(client.get(&video.baseUrl).send().unwrap());
        //println!("{:?}", video.baseUrl);
        match &response_result {
            Some(response) => {
                if response.status() == 200 {
                    println!("403");
                    break;
                }
            }
            None => return 0,
        }
    }
    let mut response: reqwest::blocking::Response = response_result.unwrap();
    fetchstream_core(response, &client, "file.mp4");

    let mut response_result: Option<reqwest::blocking::Response> = None;

    for audio in &dash.audio {
        response_result = Some(client.get(&audio.baseUrl).send().unwrap());
        //println!("{:?}", video.baseUrl);
        match &response_result {
            Some(response) => {
                if response.status() == 200 {
                    println!("403");
                    break;
                }
            }
            None => return 0,
        }
    }
    let mut response: reqwest::blocking::Response = response_result.unwrap();
    fetchstream_core(response, &client, "file.mp3");
    0
}

fn download(url: &String){
    let dash = getstream(url);
    //println!("{:#?}",dash);
    fetchstream(dash);
    let result = mix_audio_video("file.mp4", "file.mp3", "outputfile.mp4");
}


use std::process::Command;

fn mix_audio_video(video_path: &str, audio_path: &str, output_path: &str) -> std::io::Result<()> {
    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(video_path)
        .arg("-i")
        .arg(audio_path)
        .arg("-c:v")
        .arg("copy")
        .arg("-c:a")
        .arg("aac")
        .arg("-strict")
        .arg("experimental")
        .arg(output_path)
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to mix audio and video",
        ))
    }
}

fn getstream(url: &String) -> Dash{
    let mut url = Url::parse(url).expect("Failed to parse URL");

    let prefix = "/video/";
    let suffix = "/";
    let path = url.path();
    let start = prefix.len();
    let end = path.len() - suffix.len();
    let bvid = &path[start..end];
    println!("{}", bvid);

    let cookie_store = {
      if let Ok(file) = std::fs::File::open("cookies.json")
        .map(std::io::BufReader::new)
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
    let client = reqwest::blocking::Client::builder()
    .cookie_provider(std::sync::Arc::clone(&cookie_store))
    .build()
    .unwrap();

    let mut url = Url::parse("https://api.bilibili.com/x/web-interface/view").expect("Failed to parse URL");

    
    url.query_pairs_mut()
        .append_pair("bvid", &bvid);

    //println!("{:#?}", url.to_string());
    let response = client.get(&url.to_string()).send().unwrap();
    #[derive(Deserialize,Serialize, Debug)]
    struct Data {
        bvid: String,
        title: String,
        cid: i32,
    }
    let response:Response<Data>= response.json().unwrap();
    println!("{:#?}", response.data);


    let mut url = Url::parse("https://api.bilibili.com/x/player/playurl").expect("Failed to parse URL");

    //let mut url = Url::parse("https://api.bilibili.com/x/player/wbi/playurl").expect("Failed to parse URL");
    url.query_pairs_mut()
        .append_pair("bvid", &bvid);
    url.query_pairs_mut()
        .append_pair("cid", &response.data.cid.to_string());
    url.query_pairs_mut()
        .append_pair("fnval", "16");

    let response = client.get(&url.to_string()).send().unwrap();
    #[derive(Deserialize,Serialize, Debug)]
    struct Data2 {
        accept_description: Vec<String>,
        dash:Dash,
    }
    let response:Response<Data2>= response.json().unwrap();
    println!("{:#?}", response);
    response.data.dash
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exploration() {
        getVideoInfo();
    }
}
use std::fs::File;
use std::io::copy;
use reqwest::header::CONTENT_LENGTH;
use reqwest::header::RANGE;
use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom};

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Login {} => {
            let _ = login();
            return Ok(());
        }
        Commands::Download {url} => {
            println!("download video");
            download(url);
            return Ok(());
        }
    }




    // You can check the value provided by positional arguments, or option arguments
    /*
    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }
    */

    //assert!(Url::parse("http://[:asdfas::1]") == Err(ParseError::InvalidIpv6Address));
    /*
    println!("url: {:?}", cli.url);
    for item in &cli.url {
        println!("{:?}", item);
        let issue_list_url = Url::parse(item);
        println!("{:?}", issue_list_url);
    }
    */

    //let _ = login();
    //let _ = login_test();

    /*
    assert!(issue_list_url.scheme() == "https");
    assert!(issue_list_url.username() == "");
    assert!(issue_list_url.password() == None);
    assert!(issue_list_url.host_str() == Some("github.com"));
    assert!(issue_list_url.host() == Some(Host::Domain("github.com")));
    assert!(issue_list_url.port() == None);
    assert!(issue_list_url.path() == "/rust-lang/rust/issues");
    assert!(issue_list_url.path_segments().map(|c| c.collect::<Vec<_>>()) ==
            Some(vec!["rust-lang", "rust", "issues"]));
    assert!(issue_list_url.query() == Some("labels=E-easy&state=open"));
    assert!(&issue_list_url[Position::BeforePath..] == "/rust-lang/rust/issues?labels=E-easy&state=open");
    assert!(issue_list_url.fragment() == None);
    assert!(!issue_list_url.cannot_be_a_base());
    */
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



/*

    //save_cookies_to_file(&response, "./cookies.txt");
    //let theheader = response.headers();
    //println!("{:#?}", theheader);
    //let cookies = theheader.get_all("set-cookie");

    //println!("{:#?}",cookies);


    /*
    let mut iter = cookies.iter();
    loop {
        println!("{:#?}", iter.next().unwrap());
        if iter.next().is_none() {
            break;
        }
    }
    */
    //let body = response.text();
    //println!("{:#?}", body);

*/



/*
fn login_test() -> Result<(), reqwest::Error> {
    // Load an existing set of cookies, serialized as json
    let cookie_store = {
      if let Ok(file) = std::fs::File::open("cookies.json")
        .map(std::io::BufReader::new)
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
    {
      // Examine initial contents
      println!("initial load");
      let store = cookie_store.lock().unwrap();
      for c in store.iter_any() {
        println!("{:?}", c);
      }
    }


    // Build a `reqwest` Client, providing the deserialized store
    let client = reqwest::blocking::Client::builder()
    .cookie_provider(std::sync::Arc::clone(&cookie_store))
    .build()
    .unwrap();
    let response = client.get("https://api.bilibili.com/x/web-interface/nav").send()?;
    let body = response.text();
    println!("{:#?}", body);

    Ok(())

}
*/

