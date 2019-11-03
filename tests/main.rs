use const_type::
{
  Const,
};

Const!
{
  /// `Bar` is like enum, but variants might have the same value.
  pub Bar:                              u16
  {
    /// Variant A
    A = 1,
    /// Variant B
    B = 2,
    /// Variant C
    C = 3,
    /// Variant D
    D = Bar::A.0,
  }
}

impl Into < usize >                     for Bar
{
  fn into
  (
    self
  )
  ->  usize
  {
    self.0 as usize
  }
}

#[test]
fn main () -> Result<(), &'static str>
{
  let foo: Bar = Bar::C;
  Ok  ( ( ) )
}
