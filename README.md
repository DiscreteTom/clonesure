# Clonesure

[![Crates.io](https://img.shields.io/crates/l/clonesure?style=flat-square)](https://crates.io/crates/clonesure)
[![Crates.io](https://img.shields.io/crates/v/clonesure?style=flat-square)](https://crates.io/crates/clonesure)
[![docs.rs](https://img.shields.io/docsrs/clonesure?style=flat-square)](https://docs.rs/clonesure/latest)

A helper macro to create closures which will clone its environment.

## Install

Add the following line to your Cargo.toml file:

```toml
clonesure = "0.3.0"
```

## Getting Started

When define parameters of a closure, use `@var` to clone a variable, use `@mut var` to clone a mutable variable.

**Cloned variables must be in front of closure parameters.**

E.g.:

```rust
cc!(|@a, @mut b, c| a + b + c)
```

will be translated to:

```rust
{
  let a = a.clone();
  let mut b = b.clone();
  move |c| a + b + c
}
```

## Examples

```rust
use clonesure::cc;

fn main() {
  // `cc` will implicitly move its environment
  let s1 = String::from("111");
  let s2 = String::from("222");
  assert_eq!(
    // implicitly move s1 into the closure
    // brackets of the closure's body are optional
    cc!(|| s1)(),
    "111"
  );
  assert_eq!(
    // explicitly move s2 into the closure
    cc!(move || s2)(),
    "222"
  );

  // clone one var
  let s1 = String::from("111");
  assert_eq!(
    // clone s1 into the closure
    cc!(|@s1| s1)(),
    "111"
  );
  assert_eq!(
    s1, // the original s1 is still alive
    "111",
  );

  // clone many vars
  let s1 = String::from("111");
  let s2 = String::from("222");
  assert_eq!(cc!(|@s1, @s2| s1 + &s2)(), "111222");

  // clone var mut
  let s1 = String::from("111");
  let s2 = String::from("222");
  assert_eq!(
    cc!(|@mut s1, @s2| {
      s1 = s1 + &s2;
      s1
    })(),
    "111222"
  );

  // with closure params
  // cloned vars must be in front of closure params
  let s1 = String::from("111");
  let s2 = String::from("222");
  let s3 = String::from("333");
  assert_eq!(
    cc!(|@mut s1, @s2, s3| {
      s1 = s1 + &s2 + s3;
      s1
    })(&s3),
    "111222333"
  );

  // param type, param pattern, return type
  let s1 = String::from("111");
  let s2 = String::from("222");
  let s3 = String::from("333");
  let s4 = 444;
  assert_eq!(
    cc!(|@mut s1, @s2, s3: String, &s4| -> String {
      s1 = s1 + &s2 + &s3;
      format!("{}{}", s1, s4)
    })(s3, &s4),
    "111222333444"
  );
}
```

## [CHANGELOG](https://github.com/DiscreteTom/clonesure/blob/main/CHANGELOG.md)
