#[macro_use]
extern crate const_type;

Const!
{
  pub Bar: u8
  {
    A = 1,
    B = 2,
    C = 2,
    D = Bar::A.0,
  }
}

#[test]
fn main () -> Result<(), &'static str>
{
  let foo: Bar = Bar::A;
  Ok  ( ( ) )
}