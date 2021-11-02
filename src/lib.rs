/// **C**losure which will **C**lone its environment.
///
/// # Getting Started
///
/// Use `@var` to clone a variable. Use `@mut var` to clone a mutable variable.
///
/// E.g.:
///
/// ```ignore
/// cc!(|@a, @mut b, c| { a + b + c })
/// ```
///
/// will be translated to:
///
/// ```ignore
/// {
///   let a = a.clone();
///   let mut b = b.clone();
///   move |c| { a + b + c }
/// }
/// ```
///
/// # Examples
///
/// ```
/// use clonesure::cc;
///
/// fn main() {
///   // clone one var
///   let s1 = String::from("111");
///   assert_eq!(
///     cc!(|@s1| format!("{}", s1))(),
///     "111"
///   );
///  
///   // clone many vars
///   let s1 = String::from("111");
///   let s2 = String::from("222");
///   assert_eq!(
///     cc!(|@s1, @s2| format!("{} {}", s1, s2))(),
///     "111 222"
///   );
///  
///   // clone var mut
///   let s1 = String::from("111");
///   let s2 = String::from("222");
///   assert_eq!(
///     cc!(|@mut s1, @s2| {
///       s1 = s1 + &s2;
///       format!("{}", s1)
///     })(),
///     "111222"
///   );
///  
///   // with closure params
///   let s1 = String::from("111");
///   let s2 = String::from("222");
///   assert_eq!(
///     cc!(|@mut s1, @s2, s3| {
///       s1 = s1 + &s2;
///       format!("{} {}", s1, s3)
///     })("333"),
///     "111222 333"
///   );
///  
///   // order & count doesn't matter
///   let s2 = String::from("222");
///   let s3 = String::from("333");
///   let s4 = String::from("444");
///   let s5 = String::from("555");
///   assert_eq!(
///     cc!(|s1, @s2, @mut s3, @mut s4, @s5, s6| {
///       s3 = s2 + &s3;
///       s4 = s4 + &s5;
///       format!("{} {} {} {}", s1, s3, s4, s6)
///     })("111", "666"),
///     "111 222333 444555 666"
///   );
///  
///   // ref params are not supported for now, but will be added in the future
///   // cc!(|&s1, &mut s2| {})
/// }
/// ```
#[macro_export]
macro_rules! cc {
  // public interface, eat the first `|`
  (|$($t:tt)*) => {
    cc!(@impl mut[] clone[] param[] $($t)*)
  };
  // public interface, eat the leading `move |`
  (move |$($t:tt)*) => {
    cc!(@impl mut[] clone[] param[] $($t)*)
  };

  // eat `@mut xx`
  (@impl mut[$($mut:ident)*] clone[$($clone:ident)*] param[$($param:ident)*] @mut $var:ident $($t:tt)*)=>{
    cc!(@impl mut[$($mut)* $var] clone[$($clone)*] param[$($param)*] $($t)*)
  };
  // eat `,`
  (@impl mut[$($mut:ident)*] clone[$($clone:ident)*] param[$($param:ident)*] , $($t:tt)*)=>{
    cc!(@impl mut[$($mut)*] clone[$($clone)*] param[$($param)*] $($t)*)
  };
  // eat `@xx`
  (@impl mut[$($mut:ident)*] clone[$($clone:ident)*] param[$($param:ident)*] @$var:ident $($t:tt)*)=>{
    cc!(@impl mut[$($mut)*] clone[$($clone)* $var] param[$($param)*] $($t)*)
  };
  // eat `xx`
  (@impl mut[$($mut:ident)*] clone[$($clone:ident)*] param[$($param:ident)*] $var:ident $($t:tt)*)=>{
    cc!(@impl mut[$($mut)*] clone[$($clone)*] param[$($param)* $var] $($t)*)
  };
  // eat the second `|`, generate result
  (@impl mut[$($mut:ident)*] clone[$($clone:ident)*] param[$($param:ident)*] | $($t:tt)*)=>{{
    $(
      let mut $mut = $mut.clone();
    )*

    $(
      let $clone = $clone.clone();
    )*

    move |$($param),*| $($t)*
  }};
}
