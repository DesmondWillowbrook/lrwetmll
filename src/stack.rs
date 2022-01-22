use std::mem;

#[derive(Debug)]
struct Node<T> {
	val: Option<T>,
	prev: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct Stack<T> {
	head: Option<Box<Node<T>>>,
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
		eprintln!("Calling drop!");
        while let Some(mut h) = mem::take(&mut self.head) {
			mem::swap(&mut self.head, &mut h.prev);
			drop(h);
		}
    }
}

impl<T> Stack<T> {
	pub fn new () -> Self {
		Stack {head: None}
	}

	pub fn push (&mut self, val: T) {
		let mut new_head = Box::new(Node {prev: None, val: Some(val)});
		mem::swap(&mut self.head, &mut new_head.prev);
		// new_head.prev = self.head, self.head = None
		self.head = Some(new_head);
	}

	pub fn pop (&mut self) -> Option<T> {
		match mem::take(&mut self.head) {
			Some(head) => {
				self.head = head.prev;
				head.val
			},
			None => None,
		}
	}

	pub fn top (&self) -> Option<&T> {
		if let Some(h) = &self.head {
			if let Some(v) = &h.val {
				return Some(v)
			}
		}
		None
	}
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test_stack {
    use super::Stack;

	#[test]
	fn test_stack () {
		let mut s = Stack::new();
		s.push(1);
		s.push(2);
		assert_eq!(s.top(), Some(&2));
		s.push(3);
		assert_eq!(s.top(), Some(&3));
		assert_eq!(s.pop(), Some(3));
		assert_eq!(s.pop(), Some(2));

		s.push(1);
		s.push(2);
		drop(s);
	}
}