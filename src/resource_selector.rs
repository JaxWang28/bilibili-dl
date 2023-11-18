/* 资源选择器 选择下载的资源 */
use crate::target_parser::Video;
use tokio::sync::mpsc;
use reqwest::Client;
use tokio::task::JoinSet;
use url::Url;
use serde::Serialize;
use serde::Deserialize;
use tokio::join;
use reqwest::header;
pub struct ResourceSelector {
    client: &'static Client,
    receiver: mpsc::Receiver<Video>,
    sender: mpsc::Sender<(reqwest::Response, reqwest::Response, String)>,
}

impl ResourceSelector {
    pub fn new(client: &'static Client, receiver: mpsc::Receiver<Video>, sender: mpsc::Sender<(reqwest::Response, reqwest::Response, String)>) -> ResourceSelector {
        ResourceSelector{
            client, 
            receiver,
            sender,
        }
    }
    pub async fn start(mut self) {
        let mut set = JoinSet::new();
        while let Some(video) = self.receiver.recv().await {
            set.spawn(select(self.client, video, self.sender.clone()));
        }
        while let Some(_) = set.join_next().await {}
    }
}



use serde_json::Value;
async fn select(client: &Client, video: Video, sender: mpsc::Sender<(reqwest::Response, reqwest::Response, String)>) {
    println!("select");
    //let mut url = Url::parse("https://api.bilibili.com/x/player/wbi/playurl").expect("Failed to parse URL");
    let mut url = Url::parse("https://api.bilibili.com/x/player/playurl").expect("Failed to parse URL");
    if let Some(ref bvid) = video.bvid {
        url.query_pairs_mut()
            .append_pair("bvid", bvid);
    }
    if let Some(ref cid) = video.cid {
        url.query_pairs_mut()
            .append_pair("cid", cid);
    }
    url.query_pairs_mut()
        .append_pair("fnval", "16");
    //println!("{}",url.to_string());
    let response = client.get(&url.to_string()).send().await.unwrap();
    //println!("{:?}", response);
    #[derive(Deserialize, Serialize, Debug)]
    struct Response <T>{
        code: i32,
        message: String,
        ttl: i32,
        data: T,
    }
    #[derive(Deserialize,Serialize, Debug)]
    struct Dash {
        video: Vec<Obj>,
        audio: Vec<Obj>,
    }
    #[derive(Deserialize,Serialize, Debug)]
    struct Data {
        //from: String
        accept_description: Vec<String>,
        dash:Dash,
    }
    let response:Response<Data>= response.json().await.unwrap();
    //
    //let json: Value = response.json().await.unwrap();
    println!("{:#?}",response);

    let get_video_url = select_url(client, response.data.dash.video);
    let get_audio_url = select_url(client, response.data.dash.audio);

    if let (Some(video_rep), Some(audio_rep)) = join!(get_video_url, get_audio_url){
        println!("send");
        println!("video_rep:{:?}", video_rep);
        println!("audio_rep:{:?}", audio_rep);
        let _ = sender.send((video_rep, audio_rep, video.title.unwrap())).await;
    }
}




#[derive(Deserialize,Serialize, Debug)]
#[allow(non_snake_case)]
struct Obj {
    id:i32,
    baseUrl:String,
    base_url:String,
}


async fn select_url(client: &Client, obj_array: Vec<Obj>) -> Option<reqwest::Response>{
    println!("select_url");
    for obj in obj_array {
        let mut response = client.get(obj.base_url).header(header::REFERER, "https://www.bilibili.com").header(header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.3").send().await.unwrap();
        if response.status() == 200 {
            println!("Some");
            return Some(response);
        }
    }
    None
}

