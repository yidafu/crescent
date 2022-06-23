pub struct Node<T> {
  lineinfo: (u32, u32),
  inner: T,
}

impl<T> Node<T> {
  fn new(inner: T) -> Node<T> {
    Node { inner, lineinfo: (0, 0) }
  }

  fn inner(&self) -> &T {
    &self.inner
  }

  fn mut_inner(&mut self) -> &mut T {
    &mut self.inner
  }
}