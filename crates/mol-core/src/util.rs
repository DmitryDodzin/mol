pub trait ToBox {
  fn to_box(self) -> Box<Self>
  where
    Self: Sized,
  {
    Box::new(self)
  }
}
