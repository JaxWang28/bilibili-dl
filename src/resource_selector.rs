/* 资源选择器 选择下载的资源 */



use tokio::sync::mpsc;
pub async fn init_res_selector(mut rx: mpsc::Receiver<i32>) {
    while let Some(x) = rx.recv().await {
    }
}
