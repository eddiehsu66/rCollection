use std::{
    fmt::format, fs, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}, sync::{mpsc, Arc, Mutex}, thread::{self, Thread}, time::Duration
};

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool{
    pub fn new(size:usize) -> ThreadPool{
        assert!(size >0);
        let (sender,receiver) = mpsc::channel();
        
        let receiver: Arc<Mutex<mpsc::Receiver<Job>>> = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size{
            workers.push(Worker::new(id,Arc::clone(&receiver)));
        }
        ThreadPool{workers,sender:Some(sender)}
    }
    pub fn execute<F>(&self,f:F) where F: FnOnce() + Send + 'static{
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}
impl Drop for ThreadPool{
    fn drop(&mut self){
        drop(self.sender.take());
        for worker in &mut self.workers{
            println!("Shutting down worker {}",worker.id);
            
            if let Some(thread) = worker.thread.take(){
                thread.join().unwrap();
            }
        }
    }
}
struct Worker{
    id: usize,
    thread : Option<thread::JoinHandle<()>>,
}
impl Worker{
    fn new(id:usize,receiver: Arc<Mutex<mpsc::Receiver<Job>>>) ->Worker{
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
            match message{
                Ok(job) => {
                    println!("Worker {id} get a job;Executing.");
                    job();
                }
                Err(_) =>{
                    println!("Worker {id} disconnected;shutting down");
                    break;
                }
            }
            
        });
        Worker{id,thread:Some(thread)}
    }
}
pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep){
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
pub fn run_server(){
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);
    for stream in listener.incoming(){
        let stream = stream.unwrap();    
        pool.execute(||{
            handle_connection(stream)
        })
    }
    println!("Shutting down.");
}