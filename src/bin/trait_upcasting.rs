trait Proc {
    fn call(&self, message: &str);
}

trait ClonableProc: Proc {
    fn clone_proc(&self) -> Box<dyn ClonableProc + 'static>;
}
impl<P: Proc + Clone + 'static> ClonableProc for P {
    fn clone_proc(&self) -> Box<dyn ClonableProc + 'static> {
        Box::new(self.clone())
    }
}


#[derive(Clone)]
struct MyProc;
impl Proc for MyProc {
    fn call(&self, message: &str) {
        println!("Hello, {message}!")
    }
}

fn main() {
    let boxed_proc: Box<dyn Proc> = Box::new(MyProc);
    boxed_proc.call("world");

    let boxed_clonable_proc: Box<dyn ClonableProc> = Box::new(MyProc);
    let boxed_clonable_proc: Box<dyn Proc>         = unsafe {std::mem::transmute(boxed_clonable_proc)};
    boxed_clonable_proc.call("world");
}
