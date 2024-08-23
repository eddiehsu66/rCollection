use async_std::net::{TcpListener};
use async_std::task::spawn;
use futures::stream::StreamExt;
use asyncWeb::handle_connection;
#[async_std::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
    listener.incoming().for_each_concurrent(None,|tcpstream| async move{
        let tcpstream = tcpstream.unwrap();
        spawn(handle_connection(tcpstream));
    }).await;
}
