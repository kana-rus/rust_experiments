use super::{HandleFunc, Method, Handler};

pub struct RadixTreeRouter {
    GET: Node,
    POST: Node,
    PATCH: Node,
    DELETE: Node,
}
struct Node {
    patterns: Patterns,
    handler:  Option<HandleFunc>,
    children: Vec<Node>,
}
struct Patterns(
    Vec<Pattern>
);
enum Pattern {
    Str(String),
    Param,
}

impl RadixTreeRouter {
    fn new<const N: usize>(handlers: [Handler; N]) -> Self {
        let (
            mut GET,
            mut POST,
            mut PATCH,
            mut DELETE,
        ) = (
            Node::root(),
            Node::root(),
            Node::root(),
            Node::root(),
        );
        for Handler { method, route, proc } in handlers {
            match method {
                Method::GET => GET.register(route, proc),
                Method::POST => POST.register(route, proc),
                Method::PATCH => PATCH.register(route, proc),
                Method::DELETE => DELETE.register(route, proc),
            }
        }
        Self {
            GET: GET.into_radix(),
            POST: POST.into_radix(),
            PATCH: PATCH.into_radix(),
            DELETE: DELETE.into_radix(),
        }
    }
    fn search<'buf>(&self, path: &'buf str) -> Option<(&HandleFunc, Vec<&'buf str>)> {
        todo!()
    }
}
impl Node {
    fn root() -> Self {
        Self {
            patterns: Patterns::root(),
            handler:  None,
            children: vec![],
        }
    }

    fn into_radix(mut self) -> Self {
        self.radixize();
        self
    }
    fn radixize(&mut self) {
        match self.children.len() {
            0 => (),
            1 => {
                self.children[0].radixize();

            },
            _ => {
            },
        }
    }

    fn register(&mut self, route: &'static str, proc: HandleFunc) {

    }
}
impl Patterns {
    fn root() -> Self {
        Self(vec![Pattern::root()])
    }

    fn append(&mut self, child: Self) {
        if self.tail().is_str()
        && child.head().is_str() {
            self.tail_mut()
                .todo!(
                    /*
                        tail_str += head_str;
                    */
                    /*
                        for p in rem_child {
                            self.push(p)
                        }
                    */
                )
        }
    }

    fn head(&self) -> &Pattern {
        self.0.first().unwrap()
    }
    fn tail(&self) -> &Pattern {
        self.0.last().unwrap()
    }
    fn tail_mut(&mut self) -> &mut Pattern {
        self.0.last_mut().unwrap()
    }
    fn pop(&mut self) -> Pattern {
        self.0.remove(0)
    }
}
impl IntoIterator for Patterns {
    type Item = <Vec<Pattern> as IntoIterator>::Item;
    type IntoIter = <Vec<Pattern> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {self.0.into_iter()}
}
impl Pattern {
    fn root() -> Self {
        Self::Str("/".to_string())
    }

    fn is_str(&self) -> bool {
        match self {
            Self::Str(_) => true,
            Self::Param  => false,
        }
    }
    fn is_param(&self) -> bool {
        match self {
            Self::Param  => true,
            Self::Str(_) => false,
        }
    }
}
