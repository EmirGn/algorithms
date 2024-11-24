struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
  fn new(value: i32) -> Node {
      Node {
          value,
          left: None,
          right: None,
      }
  }

  fn insert(&mut self, value: i32) {
      if value < self.value {
          match self.left {
              Some(ref mut left) => left.insert(value),
              None => self.left = Some(Box::new(Node::new(value))),
          }
      } else {
          match self.right {
              Some(ref mut right) => right.insert(value),
              None => self.right = Some(Box::new(Node::new(value))),
          }
      }
  }
}

fn main() {
    let mut root = Node::new(10);
    root.insert(9);
    root.insert(15);
    root.insert(3);
    root.insert(70);
    root.insert(12);
    root.insert(18);

    println!("Binary tree created.");
}