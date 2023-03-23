#[derive(Debug)]
struct Node {
    data: i32,
    next_node: Option<Box<Node>>,
}

impl Node {
    const fn new(data: i32) -> Self {
        Self {
            data,
            next_node: None,
        }
    }
}

struct SinglyLinkedList {
    head: Option<Box<Node>>,
}

impl SinglyLinkedList {
    fn new(head: Option<Box<Node>>) -> Self {
        Self { head }
    }

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    fn len(&self) -> i32 {
        0
    }
}

fn main() {
    let mut n1 = Node::new(10);
    let n2 = Node::new(20);

    n1.next_node = Some(Box::new(n2));
    println!("{:?}", n1.next_node);
}
