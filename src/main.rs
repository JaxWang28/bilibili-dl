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
//use clap::{Parser};
//use url::{Url, ParseError,Host, Position};
use url::{Url};
use reqwest;
use serde::Deserialize;
use serde::Serialize;
use std::io::{self, Write};


//use std::fs;
//use std::collections::HashMap;




#[derive(Deserialize, Serialize, Debug)]
struct Response <T>{
    code: i32,
    message: String,
    ttl: i32,
    data: T,
}


/*
#[derive(Parser)]
#[command(author="jackson", version="0.0.1", about="A commandline program to download bilibili video.", long_about = None)]
struct Cli {
    /// URLs to download
    url: Vec<String>,
}
*/

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

fn main(){
    //let cli = Cli::parse();

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

    let _ = login();
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
