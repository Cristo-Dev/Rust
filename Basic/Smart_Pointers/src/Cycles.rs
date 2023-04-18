use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node<T> {
    value: T,
    parent: RefCell<Option<Rc<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Rc<Node<T>> {
        let node = Rc::new(Node {
            value: value,
            parent: RefCell::new(None),
        });

        *node.parent.borrow_mut() = Some(node.clone());

        node
    }
}

fn main() {
    let node1 = Node::new(1);
    let node2 = Node::new(2);

    *node1.parent.borrow_mut() = Some(node2.clone());
    *node2.parent.borrow_mut() = Some(node1.clone());

    println!("Node 1: {:?}", node1);
    println!("Node 2: {:?}", node2);
}
