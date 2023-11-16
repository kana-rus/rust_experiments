
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

    pub struct UpgradeID(usize);
    /* ======================================================== */


    #[allow(non_snake_case)] fn UpgradeStreams() -> &'static StreamLit {
        UPGRADE_STREAMS.get_or_init(StreamLit::new)
    }

    async fn reserve_upgrade() -> UpgradeID {
        struct ReserveUpgrade(&'static StreamLit);
        impl Future for ReserveUpgrade {
            type Output = UpgradeID;
            fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
                let Some(_scan_lock) = self.0.request_scan()
                    else {return std::task::Poll::Pending};

                todo!()
            }
        }

        todo!()
    }

    const _: () = {
        impl StreamLit {
            fn new() -> Self {
                Self {
                    in_scanning: AtomicBool::new(false),
                    streams:     UnsafeCell::new(Vec::new()),
                }
            }

            /// 
            fn request_scan(&self) -> Option<ScanLock> {
                self.in_scanning.compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
                    .ok().and(Some(ScanLock))
            }
        }

        struct ScanLock;
        impl Drop for ScanLock {
            fn drop(&mut self) {
                UpgradeStreams().in_scanning.store(false, Ordering::Release);
            }
        }
    };
}


#[tokio::main] async fn main() {

}
