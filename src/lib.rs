#[macro_export]
macro_rules! cc {
  // public interface, eat the first `|`
  (|$($t:tt)*) => {
    cc!(@impl mut[] clone[] param[] $($t)*)
  };
  // public interface, eat the leading `move |`
  (move |$($t:tt)*) => {
    cc!(@impl_move mut[] clone[] param[] $($t)*)
  };

  // eat `@mut xx`
  (@$callback:tt mut[$($mut:ident)*] clone[$($clone:ident)*] param[$($param:ident)*] @mut $var:ident $($t:tt)*)=>{
    cc!(@$callback mut[$($mut)* $var] clone[$($clone)*] param[$($param)*] $($t)*)
  };
  // eat `,`
  (@$callback:tt mut[$($mut:ident)*] clone[$($clone:ident)*] param[$($param:ident)*] , $($t:tt)*)=>{
    cc!(@$callback mut[$($mut)*] clone[$($clone)*] param[$($param)*] $($t)*)
  };
  // eat `@xx`
  (@$callback:tt mut[$($mut:ident)*] clone[$($clone:ident)*] param[$($param:ident)*] @$var:ident $($t:tt)*)=>{
    cc!(@$callback mut[$($mut)*] clone[$($clone)* $var] param[$($param)*] $($t)*)
  };
  // eat `xx`
  (@$callback:tt mut[$($mut:ident)*] clone[$($clone:ident)*] param[$($param:ident)*] $var:ident $($t:tt)*)=>{
    cc!(@$callback mut[$($mut)*] clone[$($clone)*] param[$($param)* $var] $($t)*)
  };
  // eat the second `|`, generate result
  (@$callback:tt mut[$($mut:ident)*] clone[$($clone:ident)*] param[$($param:ident)*] | $($t:tt)*)=>{{
    $(
      let mut $mut = $mut.clone();
    )*

    $(
      let $clone = $clone.clone();
    )*

    cc!(@$callback param[$($param)*] $($t)*)
  }};

  // callback for copy closure
  (@impl param[$($param:ident)*] $($t:tt)*) => {
    |$($param),*| $($t)*
  };
  // callback for move closure
  (@impl_move param[$($param:ident)*] $($t:tt)*) => {
    move |$($param),*| $($t)*
  };
}
