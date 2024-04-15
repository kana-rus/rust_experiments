use std::{cell::RefCell, rc::Rc};

enum T {
    A,
    B(Rc<RefCell<T>>),
}

fn traverse(t: &T) -> usize {
    #[cfg(debug_assertions)] {
        println!("called `traverse`");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    match t {
        T::A    => 0,
        T::B(r) => traverse(&r.borrow()),
    }
}

fn main() {
    let x = Rc::new(RefCell::new(T::A));
    let y = T::B(Rc::clone(&x));
    *x.borrow_mut() = y;
    println!("{}", traverse(&x.borrow()));
}
