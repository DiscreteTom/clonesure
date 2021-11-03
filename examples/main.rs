use clonesure::cc;

fn main() {
  // `cc` will implicitly move its environment
  let s1 = String::from("111");
  let s2 = String::from("222");
  assert_eq!(
    // implicitly move s1 into the closure
    cc!(|| format!("{}", s1))(),
    "111"
  );
  assert_eq!(
    // explicitly move s2 into the closure
    cc!(move || format!("{}", s2))(),
    "222"
  );

  // clone one var
  let s1 = String::from("111");
  assert_eq!(
    // clone s1 into the closure
    cc!(|@s1| format!("{}", s1))(),
    "111"
  );
  assert_eq!(
    s1, // the original s1 is still alive
    "111",
  );

  // clone many vars
  let s1 = String::from("111");
  let s2 = String::from("222");
  assert_eq!(cc!(|@s1, @s2| format!("{} {}", s1, s2))(), "111 222");

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
  let s3 = 333;
  let s4 = 444;
  let s5 = 555;
  let mut s6 = 666;
  assert_eq!(
    cc!(|@mut s1, @s2, s3, &s4, mut s5, &mut s6| {
      s1 = s1 + &s2;
      s5 = 5000 + s5;
      format!("{} {} {} {} {}", s1, s3, s4, s5, s6)
    })(s3, &s4, s5, &mut s6),
    "111222 333 444 5555 666"
  );

  // the order of cloned vars doesn't matter
  // the order of closure params will be keeped
  let s1 = 111;
  let s2 = String::from("222");
  let mut s3 = 333;
  let s4 = String::from("444");
  let s5 = 555;
  let s6 = String::from("666");
  let s7 = 777;
  assert_eq!(
    cc!(|s1, @s2, &mut s3, @mut s4, &s5, @s6, mut s7| {
      s4 = s2 + &s4 + &s6;
      s7 = 7000 + s7;
      format!("{} {} {} {} {}", s1, s3, s4, s5, s7)
    })(s1, &mut s3, &s5, s7),
    "111 333 222444666 555 7777"
  );
}
