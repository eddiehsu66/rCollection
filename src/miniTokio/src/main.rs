use std::{collections::VecDeque, future::{self, Future}, pin::Pin, sync::{Arc, Mutex}, task::{Context, Poll, Waker}, thread::{self, spawn}, time::{Duration, Instant}};
use crossbeam::channel;
use futures::task::{self, ArcWake};
use futures::future::poll_fn;
struct Delay{
    when: Instant,
    waker: Option<Arc<Mutex<Waker>>>,
}
impl Future for Delay{

    type Output = ();

    fn poll(mut self: Pin<&mut Self>,cx: &mut Context<'_>)
    -> Poll<()>{
        if let Some(waker) = &self.waker{
            let mut waker = waker.lock().unwrap();
            if !waker.will_wake(cx.waker()) {
                *waker = cx.waker().clone();
            }
        }else{
            let when = self.when;
            let waker = Arc::new(Mutex::new(cx.waker().clone()));
            self.waker = Some(waker.clone());
            thread::spawn(move ||{
                let now = Instant::now();
                if now < when{
                    thread::sleep(when - now);
                }
                let waker = waker.lock().unwrap();
                waker.wake_by_ref();
            });
        }
        if Instant::now() >= self.when{
            println!("done well");
            Poll::Ready(())
        }else{
            Poll::Pending
        }
    }
}

// type Task = Pin<Box<dyn Future<Output=()>+Send>>;
// struct MiniTokio{
//     tasks: VecDeque<Task>,
// }
struct MiniTokio{
    scheduled: channel::Receiver<Arc<Task>>,
    sender: channel::Sender<Arc<Task>>
}

impl MiniTokio {
    fn new() ->MiniTokio{
        let (sender,scheduled) = channel::unbounded();
        MiniTokio{
            scheduled,
            sender
        }
    }
    fn spawn<F>(& self,future:F) where F:Future<Output = ()>+Send+'static{
        Task::spawn(future,&self.sender);
    }
    fn run(&self){
        while let Ok(task) = self.scheduled.recv(){
            task.poll();
        }
    }
}

struct Task{
    future: Mutex<Pin<Box<dyn Future<Output = ()>+Send>>>,
    executor: channel::Sender<Arc<Task>>,
}
impl Task{
    fn schedule(self: &Arc<Self>) {
        self.executor.send(self.clone());
    }
    fn poll(self: Arc<Self>){
        let waker = task::waker(self.clone());
        let mut cx = Context::from_waker(&waker);
        let mut future = self.future.try_lock().unwrap();
        let _ = future.as_mut().poll(&mut cx);
    }
    fn spawn<F>(future:F,sender:&channel::Sender<Arc<Task>>)
    where F: Future<Output=()>+Send+'static {
        let task = Arc::new(Task{
            future:Mutex::new(Box::pin(future)),
            executor: sender.clone(),
        });
        let _ = sender.send(task);
    }
}
impl ArcWake for Task{
    fn wake_by_ref(arc_self: &Arc<Self>){
        arc_self.schedule();
    }
}

async fn delay(duration: Duration){
    let when = Instant::now() + duration;
    let delay = Delay{
        when,
        waker:None,
    };
    delay.await;
}

#[tokio::main]
async fn main(){
    let tokio = MiniTokio::new();
    tokio.spawn(async{
        delay(Duration::from_secs(5)).await;
    });
    tokio.run();
    // poll_fn(move |cx| {
    //     let mut delay = delay.take().unwrap();
    //     let res = Pin::new(&mut delay).poll(cx);
    //     assert!(res.is_pending());
    //     tokio::spawn(async move{
    //         delay.await;
    //     });
    //     Poll::Ready(())
    // }).await;
}

