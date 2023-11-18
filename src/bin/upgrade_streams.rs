mod upgrade {
    use std::future::Future;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::{sync::OnceLock, cell::UnsafeCell};
    use std::sync::Arc;
    use tokio::sync::Mutex;


    /* ======================================================== */
    pub static UPGRADE_STREAMS: OnceLock<StreamLit> = OnceLock::new();

    pub struct StreamLit {
        in_scanning: AtomicBool,
        streams:     UnsafeCell<Vec<StreamCell>>,
    } const _: () = {
        unsafe impl Sync for StreamLit {}
    };

    pub struct StreamCell {
        reserved: bool,
        stream:   Option<Arc<Mutex<Stream>>>,
    }

    pub struct Stream;

    #[derive(Clone, Copy)]
    pub struct UpgradeID(usize);
    /* ======================================================== */


    #[allow(non_snake_case)] fn UpgradeStreams() -> &'static StreamLit {
        UPGRADE_STREAMS.get_or_init(StreamLit::new)
    }

    pub async fn reserve_upgrade() -> UpgradeID {
        struct ReserveUpgrade;
        impl Future for ReserveUpgrade {
            type Output = UpgradeID;
            fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
                let Some(mut streams) = UpgradeStreams().request_reservation()
                    else {cx.waker().wake_by_ref(); return std::task::Poll::Pending};

                let id = UpgradeID(match streams.iter().position(|cell| cell.is_empty()) {
                    Some(index) => index,
                    None        => {streams.push(StreamCell::new()); streams.len()-1}
                });

                streams[id.as_usize()].reserved = true;

                std::task::Poll::Ready(id)
            }
        }

        ReserveUpgrade.await
    }

    pub async fn set_stream(id: UpgradeID, stream: Arc<Mutex<Stream>>) {
        (unsafe {UpgradeStreams().get_mut()})[id.as_usize()].stream = Some(stream);
    }

    pub async fn assume_upgradable(id: UpgradeID) -> Stream {
        struct AssumeUpgradable{id: UpgradeID}
        impl Future for AssumeUpgradable {
            type Output = Stream;
            fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
                let Some(StreamCell { reserved, stream }) = (unsafe {UpgradeStreams().get_mut()}).get_mut(self.id.as_usize())
                    else {cx.waker().wake_by_ref(); return std::task::Poll::Pending};

                if !stream.as_ref().is_some_and(|arc| Arc::strong_count(arc) == 1)
                    {cx.waker().wake_by_ref(); return std::task::Poll::Pending};

                *reserved = false;
                std::task::Poll::Ready(unsafe {
                    Mutex::into_inner(
                        Arc::into_inner(
                            Option::take(stream)
                                .unwrap_unchecked())
                                    .unwrap_unchecked())})
            }
        }

        AssumeUpgradable{id}.await
    }

    const _: () = {
        impl StreamLit {
            fn new() -> Self {
                Self {
                    in_scanning: AtomicBool::new(false),
                    streams:     UnsafeCell::new(Vec::new()),
                }
            }

            fn get(&self) -> &Vec<StreamCell> {
                unsafe {&*self.streams.get()}
            }
            unsafe fn get_mut(&self) -> &mut Vec<StreamCell> {
                &mut *self.streams.get()
            }

            /// 
            fn request_reservation(&self) -> Option<ReservationLock<'_>> {
                self.in_scanning.compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
                    .ok().and(Some(ReservationLock(unsafe {self.get_mut()})))
            }
        }

        struct ReservationLock<'scan>(&'scan mut Vec<StreamCell>);
        impl<'scan> Drop for ReservationLock<'scan> {
            fn drop(&mut self) {
                UpgradeStreams().in_scanning.store(false, Ordering::Release);
            }
        }
        impl<'scan> std::ops::Deref for ReservationLock<'scan> {
            type Target = Vec<StreamCell>;
            fn deref(&self) -> &Self::Target {
                &*self.0
            }
        }
        impl<'scan> std::ops::DerefMut for ReservationLock<'scan> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.0
            }
        }
    };

    const _: () = {
        impl StreamCell {
            fn new() -> Self {
                Self {
                    reserved: false,
                    stream:   None,
                }
            }

            fn is_empty(&self) -> bool {
                // 0. empty
                // 1. `reserved` is set to true by a client who requested scanning
                // 2. `stream` get Some(Arc<Mutex<Stream>>) later
                // 3. `Arc`'s string count will be 1
                // 4. In `poll`, call `Option::take` for (&mut stream.0)
                //    and `Arc::into_inner`, `Mutex::into_inner`,
                //    and then set `reserved` to false
                // -> 0.
                (!self.reserved) && self.stream.is_none()
            }

            fn is_just_reserved(&self) -> bool {
                self.reserved && self.stream.is_none()
            }
        }
    };

    const _: () = {
        impl UpgradeID {
            fn as_usize(self) -> usize {
                self.0
            }
        }

        impl std::fmt::Display for UpgradeID {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Display::fmt(&self.as_usize(), f)
            }
        }
    };
}


#[tokio::main] async fn main() {
    use upgrade::*;
    use std::sync::Arc;
    use tokio::sync::Mutex;


    struct Context {
        context_id: usize,
        upgrade_id: Option<UpgradeID>,
    }
    use std::sync::{OnceLock, Mutex as stdMutex};
    static CONTEXT_ID: OnceLock<stdMutex<usize>> = OnceLock::new();
    impl Context {
        fn new() -> Self {
            let mut context_id = CONTEXT_ID.get_or_init(|| stdMutex::new(0)).lock().unwrap();            
            let me = Self {
                context_id: *context_id,
                upgrade_id: None,
            };
            *context_id += 1;

            println!("[context #{}] created...", me.context_id);
            me
        }
    }

    struct Response;
    impl Response {
        async fn send(self, _stream: &mut Stream) {
            println!("[ response ] writing to stream");
        }
    }

    struct Socket {
        context: Context,
        _stream: Stream,
    } impl Socket {
        async fn new(c: Context, upgrade_id: UpgradeID) -> Self {
            let context_id = c.context_id;
            let me = Socket {
                context: c,
                _stream: assume_upgradable(upgrade_id).await
            };
            println!("===== context #{context_id} ---> socket #{upgrade_id} =====");
            me
        }
        async fn handle_messages(&self) {
            println!("[socket  #{}] handling messages",
                self.context.upgrade_id.unwrap());
        }
        async fn close(self) {
            println!("[socket  #{}] closing...",
                self.context.upgrade_id.unwrap());
        }
    }

    async fn handle(mut c: Context, handle_time: u64, before_upgrade: u64) -> (Response, Option<UpgradeID>) {let requires_upgrade = true;
        println!("[context #{}] handling", c.context_id);

        let mut upgrade_id = None;

        let response = {
            tokio::time::sleep(tokio::time::Duration::from_millis(handle_time)).await;
            Response
        };

        if requires_upgrade {
            let id = reserve_upgrade().await;
            upgrade_id   = Some(id);
            c.upgrade_id = Some(id);
        }

        if requires_upgrade {
            tokio::spawn(async move {
                let Some(id) = c.upgrade_id
                    else {panic!("Context doesn't have upgrade id")};

                let socket = {
                    tokio::time::sleep(tokio::time::Duration::from_millis(before_upgrade)).await;
                    Socket::new(c, id).await
                };

                socket.handle_messages().await;
                socket.close().await;
            });
        }

        (response, upgrade_id)
    }


    for (i, (__handle__, __before_upgrade__)) in [
        (42, 128),
        (64, 55),
        (60, 72),
        (111, 44),
    ].into_iter().enumerate() {
        let stream = Arc::new(Mutex::new(Stream));
        println!("[stream   {i}] accepted");

        match tokio::spawn({
            let stream = stream.clone();
            println!("[stream   {i}] cloned");

            async move {
                let stream = &mut *stream.lock().await;
                let (res, upgrade_id) = handle(Context::new(), __handle__, __before_upgrade__).await;
                res.send(stream).await;
                upgrade_id
            }
        }).await {
            Ok(upgrade_id) => {
                if let Some(id) = upgrade_id {
                    println!("[stream   {i}] handled: Ok(#{id})");
                    set_stream(id, stream).await;
                    println!("[stream   {i}] requested upgrade to socket #{id}...");
                }
            }
            Err(e) => eprintln!("error: {e}")
        }
    }
}
