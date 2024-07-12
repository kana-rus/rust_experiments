#![allow(unused_labels)]
fn main() {

}

struct A {
    a: Option<NotCopy1>
}

struct NotCopy1;
const NOTCOPY: NotCopy1 = NotCopy1;

impl A {
    fn f(&mut self) {}

    fn f2(&mut self) -> &NotCopy1 {
        if let Some(a_ref) = self.a.as_ref() {
            return a_ref;
        };

        //self.f();

        return &NOTCOPY;
    }

    fn f3(&mut self) -> &NotCopy1 {
        if self.a.is_some() {
            return self.a.as_ref().unwrap();
        }

        self.f();

        return &NOTCOPY;
    }

    fn f2_desugared<'a>(&'a mut self) -> &'a NotCopy1 {
        'if_let: {
            let matcher = self/* 'a での参照 */.a.as_ref();
            match matcher {
                Some(a_ref) => {
                    return a_ref/* &'a NotCopy1 */;
                }
                _ => ()
            }
        }

        /* 'a は self と同じライフタイムなので matcher による 'a での参照は生きている */

        //self.f()/* matcher が参照しているのに mutable 参照しようとしている */;

        return &NOTCOPY;
    }

    fn f3_desugared<'a>(&'a mut self) -> &'a NotCopy1 {
        'if_condition: {
            let condition = self/* 'if_condition での参照 */.a.is_some();
            if condition {
                return self.a.as_ref().unwrap();
            }
        }
        
        /* condition による 'if_condition での参照は消えている */

        self.f()/* 唯一の参照 */;

        return &NOTCOPY;
    }
}
