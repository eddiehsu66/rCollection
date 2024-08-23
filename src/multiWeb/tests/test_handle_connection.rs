use std::io::{BufReader, Read, Write};
use std::net::{TcpListener, TcpStream,Shutdown};
use std::thread;
use std::time::{Duration, Instant};
use multiWeb::{run_server, ThreadPool};
use reqwest::blocking::get;

#[test]
fn test_handle_connection() {
    thread::spawn(||{
        run_server();
    });
    thread::sleep(Duration::from_secs(1));

    // let response = get("http://127.0.0.1:7878/sleep").unwrap();
    // assert_eq!(response.status(), 200);
    let request_count = 20;
    let mut handles = vec![];
    for _ in 0..request_count {
        let handle = thread::spawn(|| {
            let start_time = Instant::now();
            let response = get("http://127.0.0.1:7878/sleep").unwrap();
            let duration = start_time.elapsed();
            (response.status(), duration.as_secs_f64())
        });

        handles.push(handle);
    }

    for handle in handles {
        match handle.join() {
            Ok((status, duration)) => {
                assert_eq!(status, 200);
                println!("Request completed in {:.2} seconds", duration);
            }
            Err(_) => {
                eprintln!("A thread panicked!");
            }
        }
    }
}