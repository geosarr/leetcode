use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct QueueNode<T> {
    value: T,
    next: Option<Box<QueueNode<T>>>,
}
impl<T> QueueNode<T> {
    pub fn new(value: T) -> Self {
        Self { value, next: None }
    }
    pub fn to_tuple(self) -> (T, Option<Box<QueueNode<T>>>) {
        (self.value, self.next)
    }
}

#[derive(Debug)]
pub struct Queue<T> {
    first: Option<Box<QueueNode<T>>>, // first to exit the queue
    len: usize,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self {
            first: None,
            len: 0,
        }
    }
    pub fn from(value: T) -> Self {
        Self {
            first: Some(Box::new(QueueNode::new(value))),
            len: 1,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn push(&mut self, value: T) {
        if self.is_empty() {
            self.first = Some(Box::new(QueueNode::new(value)));
        } else {
            let mut node = self.first.as_mut().unwrap();
            while let Some(ref mut nod) = node.next {
                node = nod;
            }
            node.next = Some(Box::new(QueueNode::new(value)));
        }
        self.len += 1;
    }
    pub fn pop(self) -> (Self, Option<T>) {
        if self.is_empty() {
            return (self, None);
        }
        let len = self.len;
        let (value, next) = self.first.map(|node| node.to_tuple()).unwrap();
        let mut new_queue = Self::new();
        new_queue.first = next;
        new_queue.len = len - 1;
        (new_queue, Some(value))
    }
}

impl<T: Display> Display for QueueNode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref node) = self.next {
            write!(f, "{} -> {}", self.value, node)
        } else {
            write!(f, "{}", self.value)
        }
    }
}
impl<T: Display> Display for Queue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref first) = self.first {
            first.fmt(f)
        } else {
            Err(std::fmt::Error)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut queue = Queue::from(0);
        queue.push(1);
        queue.push(2);
        println!("{}", queue);
        queue = queue.pop().0;
        println!("{}", queue);
        queue = queue.pop().0;
        println!("{}", queue);
        queue = queue.pop().0;
        queue.push(1);
        println!("{}", queue);
    }
}
