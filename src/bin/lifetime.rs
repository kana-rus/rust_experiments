use std::{pin::Pin, future::Future};

struct Request<'r> {
    path: &'r str,
}
type Handler<'r> =
    Box<dyn
        Fn(Request<'r>) -> Pin<
            Box<dyn
                Future<Output = ()>
                + 'r
            > 
        > + 'r
    > 
;
async fn f<'r>(_: Request<'r>) {
    println!("called")
}

fn main() {}


// ==================================================================


mod current {
    // use crate::{Request, Handler};
    // 
    // struct Router<'r> {
    //     handler: Handler<'r>
    // } impl<'r> Router<'r> {
    //     async fn handle(&'r self, request: Request<'r>) {
    //         (self.handler)(request).await
    //     }
    // }
    // 
    // fn parse<'r>(buffer: &'r [u8; 1024]) -> Request<'r> {
    //     Request {
    //         path: unsafe {std::str::from_utf8_unchecked(buffer)}
    //     }
    // }
    // 
    // async fn handle<'r>(
    //     router: &'r Router<'r>
    // ) {
    //     let buffer = [b' '; 1024];
    //     let request = parse(&buffer); // <--
    //     /*
    //         `buffer` does not live long enough
    //         borrowed value does not live long enough
    //         lifetime.rs(48, 5): `buffer` dropped here while still borrowed
    //         lifetime.rs(42, 9): lifetime `'1` appears in the type of `router`
    //         lifetime.rs(47, 9): argument requires that `buffer` is borrowed for `'1`
    //     */
    // 
    //     router.handle(request).await
    // }
}

mod fixed {
    use crate::{Request, Handler};

    struct Router<'req> {
        handler: Handler<'req>
    } impl<'req, 'router> Router<'req> {
        async fn handle(&'router self, request: Request<'req>) {
            (self.handler)(request).await
        }
    }

    fn parse<'req>(buffer: &'req [u8; 1024]) -> Request<'req> {
        Request {
            path: unsafe {std::str::from_utf8_unchecked(buffer)}
        }
    }

    async fn _handle<'req>(
        router: Router<'req>,
        buffer: &'req [u8; 1024],
    ) {
        let request = parse(buffer);
        router.handle(request).await
    }

    async fn __handle<'req, 'router>(
        router: &'router Router<'req>,
        buffer: &'req [u8; 1024],
    ) {
        let request = parse(buffer);
        router.handle(request).await
    }

    async fn ___handle<'req>(
        router: &'req Router<'req>,
        buffer: &'req [u8; 1024],
    ) {
        let request = parse(buffer);
        router.handle(request).await
    }
}
