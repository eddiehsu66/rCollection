use std::{thread::sleep, time::Duration};

use mini_redis::{client,Result};
use rand::{distributions::Alphanumeric, prelude::Distribution};
use tokio::{task, time::Instant};


#[tokio::test]
async fn main(){
    let thread_num = 100;

    let mut handles = vec![];
    let start = Instant::now();

    for i in 0..thread_num{
        let handle = task::spawn( modify_db(i));
        handles.push(handle);
    }
    for handle in handles {
        if let Err(e) = handle.await.unwrap(){
            eprintln!("Error occured:{:?}",e);
        }
    }
    let total_duration = start.elapsed();
    println!("total time is {:?}",total_duration);
    
}

async fn modify_db(data:usize) -> Result<()>{
    let mut client = client::connect("127.0.0.1:6379").await?;
    let key = generate_random_string(5).await;
    client.set(key.as_str(),data.to_string().into()).await?;

    sleep(Duration::from_secs(5));
    let result = client.get(key.as_str()).await?;
    println!("keyis {:?}test result is : {:?} and origin data is : {:?}",key,result.unwrap(),data.to_string());
    Ok(())
}

async fn generate_random_string(len: usize) -> String {
    let random_string: String = Alphanumeric
        .sample_iter(rand::thread_rng())
        .take(len)
        .map(char::from)
        .collect();
    random_string
}