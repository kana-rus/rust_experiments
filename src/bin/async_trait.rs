use experiment::sample_async_trait::{WithAsyncTrait, WithoutAsyncTrait};


struct MyStruct;

//#[async_trait::async_trait]
impl WithAsyncTrait for MyStruct {
    fn f<'life0,'async_trait>(&'life0 self) ->  core::pin::Pin<Box<dyn core::future::Future<Output = String> + core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        Box::pin(async {String::from("I'm with #[async_trait]!")})
    }        
}

impl WithoutAsyncTrait for MyStruct {
    async fn f(&self) -> String {
        String::from("I'm NOT with #[async_trait].")
    }
}


#[tokio::main]
async fn main() {
    tokio::task::spawn(async {
        let s = <MyStruct as WithAsyncTrait>::f(&MyStruct).await;
        println!("{s}");
    }).await.unwrap();
    
    tokio::task::spawn(async {
        let s = <MyStruct as WithoutAsyncTrait>::f(&MyStruct).await;
        println!("{s}");
    }).await.unwrap();
}
