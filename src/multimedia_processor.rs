use tokio::sync::mpsc;
use tokio::task::JoinSet;
pub struct MultimediaProcessor {
    receiver: mpsc::Receiver<String>,
}

impl MultimediaProcessor {
    pub fn new(receiver: mpsc::Receiver<String>) -> MultimediaProcessor {
        MultimediaProcessor {
            receiver,

        }
    }
    pub async fn start(mut self) {
        let mut set = JoinSet::new();
        while let Some(title) = self.receiver.recv().await {
            set.spawn(proc_media(title));
        }
        while let Some(_) = set.join_next().await {}
    }
}



//use std::process::Command;
use tokio::process::Command;
async fn proc_media(title: String) {
    let mut child = Command::new("ffmpeg")
        .arg("-i")
        .arg(format!("{}_video.m4s", title))
        .arg("-i")
        .arg(format!("{}_audio.m4s", title))
        /*
        .arg("-c:v")
        .arg("copy")
        .arg("-c:a")
        .arg("aac")
        .arg("-strict")
        .arg("experimental")
        */
        .arg("-c")
        .arg("copy")
        .arg("-map")
        .arg("0:v")
        .arg("-map")
        .arg("1:a")
        //.arg("-loglevel")
        //.arg("warning")
        .arg(format!("{}.mp4", title))
        .spawn()
        .unwrap();
        //.status()?;
    let status = child.wait().await.unwrap();
    println!("Command exited with status: {}", status);
}
