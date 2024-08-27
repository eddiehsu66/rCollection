use std::{collections::HashMap, hash::{self, DefaultHasher, Hash, Hasher}, sync::{Arc, Mutex}};

use bytes::Bytes;
use mini_redis::{ Command, Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

type Db = Arc<Mutex<HashMap<String,Bytes>>>;
type ShardDb = Arc<Vec<Mutex<HashMap<String,Bytes>>>>;
//ShardDb为分片锁,类似concurrentHashMap,另外后续可将Mutex可以更换为读写锁

fn new_shard_db(num_shards: usize) -> ShardDb{
    let mut db = Vec::with_capacity(num_shards);
    for _ in 0 ..num_shards{
        db.push(Mutex::new(HashMap::new()));
    }
    Arc::new(db)
}
#[tokio::main]
async fn main(){
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    let mut db = Arc::new(Mutex::new(HashMap::new()));
    // let mut db = new_shard_db(5);
    loop{
        let (socket,_) = listener.accept().await.unwrap();
        let db = db.clone();
        tokio::spawn(async move{
            process(socket,db).await;
        });
    }
}

async fn process(socket:TcpStream,db:Db){
    let mut connection = Connection::new(socket);
    while let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT:{:?}",frame);
        
        let response = match Command::from_frame(frame).unwrap() {
            Command::Set(cmd) => {
                // let key = cmd.key().to_string();
                // let mut hasher = DefaultHasher::new();
                // key.hash(&mut hasher);
                // let hash_value = hasher.finish() as usize;
                // let mut shard = db[hash_value % db.len()].lock().unwrap();
                // shard.insert(key, cmd.value().clone());
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Command::Get(cmd) =>{
                // let key = cmd.key().to_string();
                // let mut hasher = DefaultHasher::new();
                // key.hash(&mut hasher);
                // let hash_value = hasher.finish() as usize;
                // let shard = db[hash_value % db.len()].lock().unwrap();
                // if let Some(value) = shard.get(cmd.key()){
                //     Frame::Bulk(value.clone())
                // } else{
                //     Frame::Null
                // }
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()){
                    Frame::Bulk(value.clone())
                } else{
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}",cmd)
        };
        println!("RES:{:?}",response);
        connection.write_frame(&response).await.unwrap();
    }
}