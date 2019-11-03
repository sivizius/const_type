const_type
==========

Define `enum`-like `const`-types, but with aliases for variants:

```rust
  use const_type::
  {
    Const,
  };

  Const!
  {
    /// `Bar` is like enum, but variants might have the same value.
    /// `usize` is the default type, so `: usize` can be omitted in this case.
    pub Bar: usize
    {
      /// Even Variants could be and should be documented.
      A = 1,
      B = 2,
      C = 2,
      D = Bar::A.0,
    }
  }

```

This could be used like this:

```rust
  let Foo: Bar = Bar::B;
```

Because the `enum` is actually a `struct`, implementing `trait`s or methods canbe doneas usually:

```rust
impl Into < usize > for Bar
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
```
