use experiment::sample_async_trait;

struct MyStruct;

//#[async_trait::async_trait]
impl sample_async_trait::WithAsyncTrait for MyStruct {
    fn f<'life0,'async_trait>(&'life0 self) ->  core::pin::Pin<Box<dyn core::future::Future<Output = String> + core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }        
}


fn main() {

}
