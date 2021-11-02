# Clonesure

![Crates.io](https://img.shields.io/crates/l/clonesure?style=flat-square)
![Crates.io](https://img.shields.io/crates/v/clonesure?style=flat-square)
![docs.rs](https://img.shields.io/docsrs/clonesure?style=flat-square)

A helper macro to create closures which will clone its environment.

## Getting Started

Use `@var` to clone a variable. Use `@mut var` to clone a mutable variable.

E.g.:

```rust
cc!(|@a, @mut b, c| { a + b + c })
```

will be translated to:

```rust
{
  let a = a.clone();
  let mut b = b.clone();
  move |c| { a + b + c }
}
```

## Examples

```rust
use clonesure::cc;

fn main() {
  // clone one var
  let s1 = String::from("111");
  assert_eq!(
    cc!(|@s1| format!("{}", s1))(),
    "111"
  );

  // clone many vars
  let s1 = String::from("111");
  let s2 = String::from("222");
  assert_eq!(
    cc!(|@s1, @s2| format!("{} {}", s1, s2))(),
    "111 222"
  );

  // clone var mut
  let s1 = String::from("111");
  let s2 = String::from("222");
  assert_eq!(
    cc!(|@mut s1, @s2| {
      s1 = s1 + &s2;
      format!("{}", s1)
    })(),
    "111222"
  );

  // with closure params
  let s1 = String::from("111");
  let s2 = String::from("222");
  assert_eq!(
    cc!(|@mut s1, @s2, s3| {
      s1 = s1 + &s2;
      format!("{} {}", s1, s3)
    })("333"),
    "111222 333"
  );

  // order & count doesn't matter
  let s2 = String::from("222");
  let s3 = String::from("333");
  let s4 = String::from("444");
  let s5 = String::from("555");
  assert_eq!(
    cc!(|s1, @s2, @mut s3, @mut s4, @s5, s6| {
      s3 = s2 + &s3;
      s4 = s4 + &s5;
      format!("{} {} {} {}", s1, s3, s4, s6)
    })("111", "666"),
    "111 222333 444555 666"
  );

  // ref params are not supported for now, but will be added in the future
  // cc!(|&s1, &mut s2| {})
}
```
