use std::rc::Rc;

#[derive(Debug)]
struct Node<T> {
    value: T,
    children: Vec<Rc<Node<T>>>,
}

fn main() {
    let root = Rc::new(Node {
        value: 42,
        children: Vec::new(),
    });

    let child1 = Rc::new(Node {
        value: 37,
        children: Vec::new(),
    });

    let child2 = Rc::new(Node {
        value: 64,
        children: Vec::new(),
    });

    root.children.push(child1.clone());
    root.children.push(child2.clone());

    println!("Root: {:?}", root);
    println!("Child 1: {:?}", child1);
    println!("Child 2: {:?}", child2);
}
