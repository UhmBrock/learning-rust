use std::mem;

pub struct List<T> {
  head: Link<T>
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> { 
  elem: T,
  next: Link<T>
}

impl<T> List<T> {
  pub fn new() -> Self {
    return List { head: None };
  }

  pub fn push(&mut self, elem: T) {
    let new_node = Box::new(Node {
      elem: elem,
      next: self.head.take(),
    });

    self.head = Some(new_node);
  }

  pub fn pop(&mut self) -> Option<T> {
    return self.head.take().map(|node| {
      self.head = node.next;
      return node.elem;
    });
  }

  pub fn peek(&self) -> Option<&T> {
    return self.head.as_ref().map(|node| {
      return &node.elem;
    });
  }

  pub fn peek_mut(&mut self) -> Option<&mut T> {
    return self.head.as_mut().map(|node| {
      return &mut node.elem;
    });
  }

  pub fn into_iter(self) -> IntoIter<T> {
    return IntoIter(self);
  }

  // Declare a fresh lifetime for the exact borrow that creates the iter
  // So self has to be valid as long as iter is valid
  // Note: you can actually remove it on this function since the compiler can ellide the lifetimes
  //    given that it's a single input and output lifetime
  pub fn iter<'a>(&'a self) -> Iter<'a, T> {
    return Iter { next: self.head.as_deref() };
  }

  // '_ = "Intentionally Elided Lifetime", you don't have to write it
  pub fn iter_mut(&mut self) -> IterMut<'_, T> {
    return IterMut { next: self.head.as_deref_mut()};
  }
}

impl<T> Drop for List<T> {
  fn drop(&mut self) {
    let mut cur_link = self.head.take();
    
    while let Some(mut boxed_node) = cur_link {
      cur_link = mem::replace(&mut boxed_node.next, None)
    }
  }
}

// IntoIter -> T
pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
  type Item = T;
  fn next(&mut self) -> Option<Self::Item> {
    // access fields of a tuple struct numerically
    return self.0.pop();
  }
}

// Iter -> &T
pub struct Iter<'a, T> {
  next: Option<&'a Node<T>>
}

impl<'a, T> Iterator for Iter<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<Self::Item> {
    self.next.map(|node| {
      self.next = node.next.as_deref();
      return &node.elem;
    })
  }
}

// IterMut -> &mut T
pub struct IterMut<'a, T> {
  next: Option<&'a mut Node<T>>
}

impl<'a, T> Iterator for IterMut<'a, T> {
  type Item = &'a mut T;

  fn next(&mut self) -> Option<Self::Item> {
    self.next.take().map(|node| {
      self.next = node.next.as_deref_mut();
      return &mut node.elem;
    })
  }
}


#[cfg(test)]
mod test {
  use super::List;

  #[test]
  fn basics() {
    let mut list = List::new();

    // Check empty list behaves right, returns Option::None
    assert_eq!(list.pop(), None);

    // Populate list
    list.push(1);
    list.push(2);
    list.push(3);

    // Check normal removal, returns Option::Some<i32>
    assert_eq!(list.pop(), Some(3));
    assert_eq!(list.pop(), Some(2));

    // Push some more for extra testing
    list.push(4);
    list.push(5);

    // Check normal removal
    assert_eq!(list.pop(), Some(5));
    assert_eq!(list.pop(), Some(4));

    // Check exhaustion
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), None);
  }

  #[test]
  fn peek() {
    let mut list = List::new();

    assert_eq!(list.peek(), None);
    assert_eq!(list.peek_mut(), None);
    
    list.push(1); 
    list.push(2); 
    list.push(3);

    assert_eq!(list.peek(), Some(&3));
    assert_eq!(list.peek_mut(), Some(&mut 3));

    // the closure does not actually define the variable
    // it only defines a patternt that matches the variable 
    // and then copies the value into the variable
    // so instead of &mut value, just write value
    list.peek_mut().map(|value| {
      *value = 42
    });

    assert_eq!(list.peek(), Some(&42));
    assert_eq!(list.pop(), Some(42));
  }

  #[test]
  fn into_iter() {
    let mut list = List::new();

    list.push(1);
    list.push(2);
    list.push(3);

    let mut iter = list.into_iter();
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), None);
  }

  #[test]
  fn iter() {
    let mut list = List::new();

    list.push(1);
    list.push(2);
    list.push(3);

    let mut iter = list.iter();

    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), None);
  }

  #[test]
  fn iter_mut() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);

    let mut iter = list.iter_mut();
    assert_eq!(iter.next(), Some(&mut 3 ));
    assert_eq!(iter.next(), Some(&mut 2 ));
    assert_eq!(iter.next(), Some(&mut 1 ));
    assert_eq!(iter.next(), None);
  }
}