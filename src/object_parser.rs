
use tokio::sync::oneshot;

use tokio::sync::mpsc;
use url::{Url};


pub enum Object{
    Url(Vec<String>),
    Bvid,
}

// https://www.bilibili.com/video/BV1X94y137HR/?spm_id_from=333.1007.tianma.2-1-4.click
// https://www.bilibili.com/video/BV1Eb411u7Fw?p=5


/* start url parser */
/* way: bid vid ? ....*/
pub async fn init_object_parser(object: Object, page_start: u8, page_end: u8, tx: mpsc::Sender<i32>){
    match object {
        Object::Url(urls) => {
            for url in urls {
                let tx_clone = tx.clone();
                tokio::spawn(url_parser(url, page_start, page_end, tx_clone));
            }

        },
        Object::Bvid => todo!()

    }

}


struct bctp {
    bvid: String, 
    cid: String,
    titile: String,
}


async fn url_parser(url: String, page_start: u8, page_end: u8, tx: mpsc::Sender<i32>) {
    println!("_______________________");
    /* bvid cid title page
     *
     * */

    let url = Url::parse(&url).expect("Failed to parse URL");
    let mut segments = url.path_segments().ok_or_else(|| "cannot be base").unwrap();
    assert_eq!(segments.next(), Some("video"));
    let bvid = match segments.next() {
        Some(x) => x,
        None => "",
    };
    println!("{}", bvid);
}


async fn get_bvid(url: String) {

    todo!();
}


