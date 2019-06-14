const_type
==========

define enum-like const-types, but with aliases for variants:

```rust
  const!
  {
    pub Bar: usize
    {
      A = 1,
      B = 2,
      C = 2,
      D = Bar::A.0,
    }
  }

```

this can be used like this:

```rust
  let Foo: Bar = Bar::B;
```