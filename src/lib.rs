/// **C**losure which will **C**lone its environment.
///
/// # Getting Started
///
/// When define parameters of a closure, use `@var` to clone a variable, use `@mut var` to clone a mutable variable.
///
/// **Cloned variables must be in front of closure parameters.**
///
/// E.g.:
///
/// ```ignore
/// cc!(|@a, @mut b, c| a + b + c)
/// ```
///
/// will be translated to:
///
/// ```ignore
/// {
///   let a = a.clone();
///   let mut b = b.clone();
///   move |c| a + b + c
/// }
/// ```
///
/// # Examples
///
/// ```
/// use clonesure::cc;
///
/// fn main() {
///   // `cc` will implicitly move its environment
///   let s1 = String::from("111");
///   let s2 = String::from("222");
///   assert_eq!(
///     // implicitly move s1 into the closure
///     // brackets of the closure's body are optional
///     cc!(|| s1)(),
///     "111"
///   );
///   assert_eq!(
///     // explicitly move s2 into the closure
///     cc!(move || s2)(),
///     "222"
///   );
///
///   // clone one var
///   let s1 = String::from("111");
///   assert_eq!(
///     // clone s1 into the closure
///     cc!(|@s1| s1)(),
///     "111"
///   );
///   assert_eq!(
///     s1, // the original s1 is still alive
///     "111",
///   );
///
///   // clone many vars
///   let s1 = String::from("111");
///   let s2 = String::from("222");
///   assert_eq!(cc!(|@s1, @s2| s1 + &s2)(), "111222");
///
///   // clone var mut
///   let s1 = String::from("111");
///   let s2 = String::from("222");
///   assert_eq!(
///     cc!(|@mut s1, @s2| {
///       s1 = s1 + &s2;
///       s1
///     })(),
///     "111222"
///   );
///
///   // with closure params
///   // cloned vars must be in front of closure params
///   let s1 = String::from("111");
///   let s2 = String::from("222");
///   let s3 = String::from("333");
///   assert_eq!(
///     cc!(|@mut s1, @s2, s3| {
///       s1 = s1 + &s2 + s3;
///       s1
///     })(&s3),
///     "111222333"
///   );
///
///   // param type, param pattern, return type
///   let s1 = String::from("111");
///   let s2 = String::from("222");
///   let s3 = String::from("333");
///   let s4 = 444;
///   assert_eq!(
///     cc!(|@mut s1, @s2, s3: String, &s4| -> String {
///       s1 = s1 + &s2 + &s3;
///       format!("{}{}", s1, s4)
///     })(s3, &s4),
///     "111222333444"
///   );
/// }
/// ```
#[macro_export]
macro_rules! cc {
  // simple closures without params
  // cc will implicitly move its environment
  (|| $($t:tt)*) => {
    move || $($t)*
  };
  (move || $($t:tt)*) => {
    move || $($t)*
  };

  // public interface, eat the first `|`
  (|$($t:tt)*) => {
    cc!(@@impl mut[] clone[] $($t)*)
  };
  // public interface, eat the leading `move |`
  (move |$($t:tt)*) => {
    cc!(@@impl mut[] clone[] $($t)*)
  };

  // eat `@mut xx`, store in the array `mut`
  (@@impl mut[$($mut:ident)*] clone[$($clone:ident)*] @mut $var:ident $($t:tt)*)=>{
    cc!(@@impl mut[$($mut)* $var] clone[$($clone)*] $($t)*)
  };
  // eat `@xx`, store in the array `clone`
  (@@impl mut[$($mut:ident)*] clone[$($clone:ident)*] @$var:ident $($t:tt)*)=>{
    cc!(@@impl mut[$($mut)*] clone[$($clone)* $var] $($t)*)
  };
  // eat `,`
  (@@impl mut[$($mut:ident)*] clone[$($clone:ident)*] , $($t:tt)*)=>{
    cc!(@@impl mut[$($mut)*] clone[$($clone)*] $($t)*)
  };
  // otherwise, generate result
  (@@impl mut[$($mut:ident)*] clone[$($clone:ident)*] $($t:tt)*)=>{{
    $(
      let mut $mut = $mut.clone();
    )*

    $(
      let $clone = $clone.clone();
    )*

    move |$($t)*
  }};
}
