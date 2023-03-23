use tokio::time::sleep;
use std::{
    pin::pin,
    task::Poll,
    time::Duration,
    future::{Future, IntoFuture},
};


struct AsyncOption<T: Clone>(
    Option<T>
);
impl<T: Clone> AsyncOption<T> {
    fn some(value: T) -> Self {
        Self(Some(value))
    }
    async fn get(&self) -> Option<T> {
        // sleep(Duration::from_secs(1)).await;
        std::thread::sleep(Duration::from_secs(1));
        self.0.clone()
    }
}
impl<T: Clone> Future for AsyncOption<T> {
    type Output = Option<T>;
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        pin!(self.get()).poll(cx)
    }
}


struct Timer {
    id:   u8,
    time: u8,
}
impl Timer {
    fn new(id: u8) -> Self {
        Self { id, time: 0 }
    }
    async fn tick(&mut self) {
        if self.time == self.id {
            return
        } else {
            // sleep(Duration::from_secs(1)).await;
            std::thread::sleep(Duration::from_secs(1));
            println!("timer {}: ticked [{} -> {}]",
                self.id,
                self.time,
                self.time + 1,
            );
            self.time += 1
        }
    }
    fn is_ready(&self) -> bool {
        println!("`is_ready` was called by Timer {{id:{}, time:{}}}", self.id, self.time);
        self.time == self.id
    }
}
impl IntoFuture for Timer {
    type Output = u8;
    type IntoFuture = TimerFuture;
    fn into_future(self) -> Self::IntoFuture {
        TimerFuture {timer: self}
    }
}

struct TimerFuture {
    timer: Timer,
}
impl Future for TimerFuture {
    type Output = u8;
    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        if self.timer.is_ready() {
            Poll::Ready(self.timer.id)
        } else {
            match pin!(self.timer.tick()).poll(cx) {
                Poll::Pending  => {/*cx.waker().wake_by_ref();*/ dbg!(Poll::Pending)},
                Poll::Ready(_) => {cx.waker().wake_by_ref(); dbg!(Poll::Pending)},
            }
        }
    }
}


#[tokio::main]
async fn main() {
    // let option = AsyncOption::some("I am option");
    // println!("{}", option.await.unwrap());


    // let timers = [
    //     Timer::new(2),
    //     Timer::new(3),
    //     Timer::new(5),
    //     Timer::new(7),
    //     Timer::new(11),
    // ];
    // 
    // for timer in timers {
    //     let id = timer.await;
    //     println!("{id}")
    // }

    let id = Timer::new(2).await;
    println!("{id}");
}
