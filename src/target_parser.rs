use reqwest::Client;
use tokio::sync::mpsc;
use url::{Url};
use serde::Serialize;
use reqwest::Response;
use serde::Deserialize;
use tokio::task::JoinSet;
use tokio::task; 
use crate::Target::VIDEO;
use crate::Target::LIVE;

#[derive(Debug)]
pub struct Video{
    pub aid: Option<String>,
    pub bvid: Option<String>,
    pub cid: Option<String>,
    pub flags: Option<u8>,
    pub title: Option<String>,
    pub page_range: (usize, usize),
    pub page_id :Option<u8>,
}

#[derive(Debug)]
pub enum Target {
    /* 普通视频 */
    VIDEO(Video),
    /* 直播 */
    LIVE,
}

pub struct TargetParser{
    client: &'static Client,
    receiver: mpsc::Receiver<Target>,
    sender: mpsc::Sender<Video>,
}


impl TargetParser {
    pub fn new(client: &'static Client, receiver: mpsc::Receiver<Target>, sender: mpsc::Sender<Video>) -> TargetParser{
        TargetParser {
            client,
            receiver,
            sender
        }
    }
    pub async fn start(mut self) {
        let mut set = JoinSet::new();
        while let Some(target) = self.receiver.recv().await {
            set.spawn(target_parse(self.client, target, self.sender.clone()));
            //target_parse(self.client, target).await;

        }
        while let Some(_) = set.join_next().await {}
    }
}

async fn target_parse(client: &Client, target: Target, sender: mpsc::Sender<Video>) {
    match target {
        VIDEO(video) => {
            proc_video(client, video, sender).await;
        },
        LIVE => {
            proc_live();
        }
    }
}

async fn proc_video(client: &Client, mut video:Video, sender: mpsc::Sender<Video>) {
    let mut url = Url::parse("https://api.bilibili.com/x/web-interface/view").expect("Failed to parse URL");

    if let Some(ref bvid) = video.bvid {
        url.query_pairs_mut()
            .append_pair("bvid", bvid);
    }
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
    struct Page {
        cid: i32,
    }
    #[derive(Deserialize,Serialize, Debug)]
    struct Data {
        bvid: String,
        videos: i32,
        title: String,
        cid: i32,
        pages: Vec<Page>,
    }
    let response:Response<Data>= response.json().await.unwrap();

    //video.title = Some(response.data.title);
    let (page_start, page_end) = video.page_range;
    if page_start == page_end {
        video.title = Some(response.data.title);
        video.cid = Some(response.data.pages[page_start - 1].cid.to_string());
        sender.send(video).await.unwrap();
    }
    else {
        for i in page_start..=page_end {
            todo!();
        }
    }
}

fn proc_live(){
    todo!();
}

