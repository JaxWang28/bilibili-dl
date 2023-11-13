
use tokio::sync::oneshot;

use tokio::sync::mpsc;
use url::{Url};


use tokio::task::JoinSet;
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
            let mut set = JoinSet::new();
            for url in urls {
                let tx_clone = tx.clone();
                set.spawn(url_parser(url, page_start, page_end, tx_clone));
            }
            while let Some(_) = set.join_next().await {}
        },
        Object::Bvid => todo!()
    }
    
    /* 可以没有自动释放 */
    /* 必须释放，否则 res_selector 会一直阻塞 */
    //drop(tx);
}


struct bctp {
    bvid: String, 
    cid: String,
    titile: String,
}

/* page_start == 0 default page_end == 0 标识最大 */
async fn url_parser(url: String, mut page_start: u8, mut page_end: u8, tx: mpsc::Sender<i32>) {
    /* bvid cid title page
     *
     * */
    let url = Url::parse(&url).expect("Failed to parse URL");
    let mut segments = url.path_segments().ok_or_else(|| "cannot be base").unwrap();
    assert_eq!(segments.next(), Some("video"));
    let bvid = match segments.next() {
        Some(x) => x,
        None => {
            println!("no bvid");
            return
        }
    };
    if page_start == 0 {
        for (key, value) in url.query_pairs(){
            if key == "p" {
                page_end = match value.parse() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("{}", value);
                        eprintln!("无法将字符串转换为数字");
                        return
                    }
                };
                page_start = page_end;
                break;
            }
        }
        if page_start == 0 {
            page_start = 1;
            page_end = 1;
        }
    }

}


async fn get_bvid(url: String) {

    todo!();
}


