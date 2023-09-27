use std::rc::Rc;


pub struct SkipList<K:Ord, V> {
    head: Node<K, V>,
} impl<K:Ord, V> SkipList<K, V> {
    pub fn search(&self, key: &K) -> Option<&V> {
        self.head.search(key)
    }
    pub fn insert(&mut self, key: K, value: V) {
        self.head.insert(key, value)
    }
    pub fn remove(&mut self, key: &K) -> Option<V> {
        // if &self.head.key == key {
        //     if let Some(n) = self.head.next {
// 
        //     } else {
        //         match &mut *self.head.child {
        //             
        //         }
        //     }
        // } else {
        //     self.head.remove(key)
        // }
        todo!()
    }
}

enum Element<K:Ord, V> {
    Node(Node<K, V>),
    Leaf(V),
}

struct Node<K:Ord, V> {
    key:   K,
    next:  Option<Rc<Node<K, V>>>,
    child: Rc<Element<K, V>>,
} impl<K:Ord, V> Node<K, V> {
    fn search(&self, key: &K) -> Option<&V> {
        match self.goto_eq_or_lt(key) {
            SearchNodeResult::NotFound => None,
            SearchNodeResult::Eq(n)    => Some(n.goto_leaf()),
            SearchNodeResult::Lt(n)    => match &*n.child {
                Element::Leaf(_) => None,
                Element::Node(n) => n.search(key),
            }
        }
    }

    fn insert(&mut self, key: K, value: V) {
        todo!()
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        todo!()
    }
} impl<K:Ord, V> Node<K, V> {
    fn goto_eq_or_lt(&self, key: &K) -> SearchNodeResult<K, V> {
        if key == &self.key {return SearchNodeResult::Eq(self)}
        if key  > &self.key {return SearchNodeResult::NotFound}

        if let Some(next) = &self.next {
            if key > &next.key {
                SearchNodeResult::Lt(self)
            } else {
                next.goto_eq_or_lt(key)
            }
        } else {
            SearchNodeResult::Lt(self)
        }
    }

    //fn goto_first_eq(&mut self, key: &K) -> Option<>

    fn goto_leaf(&self) -> &V {
        match &*self.child {
            Element::Leaf(v) => v,
            Element::Node(n) => n.goto_leaf(),
        }
    }
}

enum SearchNodeResult<'n, K:Ord, V> {
    Eq(&'n Node<K, V>),
    Lt(&'n Node<K, V>),
    NotFound,
}
