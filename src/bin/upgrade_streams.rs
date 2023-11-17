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

    async fn reserve_upgrade() -> UpgradeID {
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

    async fn set_stream(id: UpgradeID, stream: Arc<Mutex<Stream>>) {
        struct EnsureReserved{id: UpgradeID}
        impl Future for EnsureReserved {
            type Output = ();
            fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
                let Some(cell) = UpgradeStreams().get().get(self.id.as_usize())
                    else {cx.waker().wake_by_ref(); return std::task::Poll::Pending};
                
                if !cell.is_just_reserved() {
                    cx.waker().wake_by_ref(); return std::task::Poll::Pending;
                }

                std::task::Poll::Ready(())
            }
        }

        EnsureReserved{id}.await;
        (unsafe {UpgradeStreams().get_mut()})[id.as_usize()].stream = Some(stream);
    }

    async fn assume_upgradable(id: UpgradeID) -> Stream {
        struct AssumeUpgradable{id: UpgradeID}
        impl Future for AssumeUpgradable {
            type Output = Stream;
            fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
                let Some(StreamCell { reserved, stream }) = (unsafe {UpgradeStreams().get_mut()}).get_mut(self.id.as_usize())
                    else {cx.waker().wake_by_ref(); return std::task::Poll::Pending};

                if !stream.as_ref().is_some_and(|arc| Arc::strong_count(&arc) == 1) {
                    cx.waker().wake_by_ref(); return std::task::Poll::Pending;
                }

                let stream = unsafe {Mutex::into_inner(Arc::into_inner(stream.take().unwrap_unchecked()).unwrap_unchecked())};
                *reserved  = false;

                std::task::Poll::Ready(stream)
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
    };
}


#[tokio::main] async fn main() {

}
