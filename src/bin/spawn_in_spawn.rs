use std::cell::UnsafeCell;
use std::sync::{Arc, OnceLock};
use std::future::Future;


struct Context {
    data: Arc<&'static str>,
} impl Context {
    fn new() -> Self {
        Self {
            data: Arc::new("Hello, world!"),
        }
    }
    fn count(&self) -> usize {
        Arc::strong_count(&self.data)
    }
    fn into_inner(self) -> &'static str {
        Arc::into_inner(self.data).unwrap()
    }
} const _: () = {
    impl Clone for Context {
        fn clone(&self) -> Self {
            let current_count = self.count();
            let cloned = Self {
                data: self.data.clone(),
            };
            println!("[==== cloned ====] count: {} -> {}",
                current_count,
                self.count());
            cloned
        }
    }
};


static STORE: SocketStore = SocketStore::new();
struct SocketStore { lock:    OnceLock<UnsafeCell<SocketCell>> }
struct SocketCell  { context: Option<Context> }
struct Socket      { message: &'static str }

#[allow(non_snake_case)] fn StoredCell() -> &'static mut SocketCell {
    unsafe {
        &mut *STORE.lock
            .get_or_init(|| UnsafeCell::new(SocketCell{context:None}))
            .get()
    }
}

unsafe impl Sync for SocketStore {}

impl SocketStore {
    const fn new() -> Self {
        Self {
            lock: OnceLock::new(),
        }
    }
}

impl Future for SocketCell {
    type Output = Socket;
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let this = &mut self.get_mut().context;
        match this.as_ref().map(|c| c.count()) {
            Some(1) => std::task::Poll::Ready(Socket {message: this.take().unwrap().into_inner()}),
            Some(c) => {println!("Pending:  {c}"); cx.waker().wake_by_ref(); std::task::Poll::Pending}
            None    => {println!("Pending: None"); cx.waker().wake_by_ref(); std::task::Poll::Pending}
        }
    }
}

async fn store(c: Context) {
    println!("[store] count = {}", c.count());
    StoredCell().context = Some(c);
    println!("[store] done");
}
async fn assume_ready_socket() -> Socket {
    StoredCell().await
}


#[tokio::main] async fn main() {
    let c = Context::new();
    println!("[init] count = {}", c.count());

    match tokio::spawn({
        let c2 = c.clone();

        async move {
            println!("[ c2 ] count = {}", c2.count());
            handle(c2).await
        }
    }).await {
        Ok(flag) => {
            println!("[ ok ] count = {}", c.count());
            if flag {
                store(c).await;
            }
        }
        Err(e) => {
            eprintln!("error: {e}");
        }
    }
}

async fn handle(c2: Context) -> bool {
    println!("[c2 in handle] count = {}", c2.count());

    tokio::spawn(async move {
        let socket = assume_ready_socket().await;
        println!("[socket] {}", socket.message);
    });

    println!("[handle] returning true");
    true
}
