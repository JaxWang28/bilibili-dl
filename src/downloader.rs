/* 下载器
 * 资源下载的核心实现 */

use crate::target_parser::Video;
use tokio::sync::mpsc;
use reqwest::Client;
use tokio::task::JoinSet;
use url::Url;
use serde::Serialize;
use serde::Deserialize;
use tokio::join;
use reqwest::header::CONTENT_LENGTH;
use std::io::copy;
use reqwest::header::RANGE;
use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom};
use reqwest::header;
use tokio::{fs::File, io::AsyncWriteExt};


pub struct Downloader {
    client: &'static Client,
    receiver: mpsc::Receiver<(reqwest::Response, reqwest::Response, String)>,
    sender: mpsc::Sender<String>,
}

impl Downloader {
    pub fn new(client: &'static Client, receiver: mpsc::Receiver<(reqwest::Response, reqwest::Response, String)>, sender: mpsc::Sender<String>) -> Downloader {
        Downloader {
            client,
            receiver,
            sender,
        }
    }

    pub async fn start(mut self) {
        let mut set = JoinSet::new();
        while let Some((video_rep, audio_rep, title)) = self.receiver.recv().await {
            /*
            set.spawn(download(self.client, video_rep, format!("{}_video.m4s", title)));
            set.spawn(download(self.client, audio_rep, format!("{}_audio.m4s", title)));
            */

            let sender_clone = self.sender.clone();
            set.spawn(async move {
                let v_task = download(self.client, video_rep, format!("{}_video.m4s", title));
                let a_task = download(self.client, audio_rep, format!("{}_audio.m4s", title));
                join!(v_task, a_task);
                sender_clone.send(title).await;
            });
        }
        while let Some(_) = set.join_next().await {}
    }
}

async fn download(client: &Client, mut response: reqwest::Response, filename: String) {
    let response = client.get(response.url().to_string())
        .header(header::REFERER, "https://www.bilibili.com")
        .header(header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.3")
        .send().await.unwrap();
    let mut file = File::create(filename).await.unwrap();
    let content = response.bytes().await.unwrap();
    file.write_all(&content).await.unwrap();
}
