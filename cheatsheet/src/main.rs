fn main() {
  let x: (i32, f64, u8) = (500, 6.4, 1);

  let five_hundred = x.0;

  let six_point_four = x.1;

  let one = x.2;

  println!("({}, {}, {})", five_hundred, six_point_four, one);

  // Functions
  fn five() -> i32 {
    5
  }

  let x = five();

  println!("The value of x is: {}", x);

  let y = plus_one(5);

  println!("The value of y is: {}", y);

  fn plus_one(x: i32) -> i32 {
    x + 1
  }

  let number = 3;

  if number < 5 {
    println!("condition was true");
  }
  else {
    println!("condition was false");
  }

  // Ownership example
  let s = String::from("hello");

  takes_ownership(s);

  // variable s goes out of scope as String has Drop trait

  fn takes_ownership(some_string: String) {
    println!("{}", some_string);
  }

  // References and Borrowing
  let hello = String::from("hello");

  let len = calculate_length(&hello);

  println!("length of {} is {}", hello, len);

  fn calculate_length(str: &String) -> usize {
    str.len()
  }

  // The Slice Type
  let hello = "hello";
  // behind the scene equals
  let hello = &String::from("hello")[..];

  // Defining and Instantiating Structs
  struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
  }

  let user1 = User {
    username: String::from("Vasu"),
    email: String::from("vasuadari@vasuadari.com"),
    sign_in_count: 1,
    active: true
  };

  let mut user2 = User {
    username: String::from("Vasu"),
    email: String::from("vasuadari@vasuadari.com"),
    sign_in_count: user1.sign_in_count,
    ..user1
  };

  user2.username = String::from("Vasu Adari");

  fn build_user(email: String, username: String) -> User {
    User {
      username,
      email,
      sign_in_count: 1,
      active: true
    }
  }

  let new_user = build_user(
    String::from("vasuadari@vasuadari.com"),
    String::from("Vasu Adari")
  );

  #[derive(Debug)]
  struct Color(i32, i32, i32);
  #[derive(Debug)]
  struct Point(i32, i32, i32);

  let black = Color(0, 0, 0);
  let origin = Point(0, 0, 0);

  println!("{:?}", black);
  println!("{:?}", origin);

  // Option
  let some_number = Some(5);
  let some_string = Some("a string");

  let absent_number: Option<i32> = None;

  let x: i8 = 5;
  let y: Option<i8> = Some(5);

  // let sum = x + y; // this line would fail uncomment to check

  // if let
  let some_u8_value = Some(0u8);
  match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
  }

  if let Some(0u8) = some_u8_value {
    println!("[if let] zero");
  }

  // Vectors
  let mut v: Vec<i32> = Vec::new();
  v.push(1);
  v.push(4);
  v.push(6);

  let v = vec![1, 2, 3, 4, 5];
  // let does_not_exist = &v[100]; // rust panics
  let does_not_exist = v.get(100);

  let mut v = vec![1, 2, 3, 4, 5];
  let first = &v[0];
  // v.push(6); // this would fail as immutable reference exists already

  println!("The first element is: {}", first);

  let v = vec![100, 32, 57];
  for i in &v {
    println!("{}", i);
  }

  let mut v = vec![100, 32, 57];
  for i in &mut v {
    *i += 50;
  }

  // Storing UTF-8 Encoded Text with Strings
  let mut s = String::new();

  // String literal
  let data = "initial contents";

  let s = data.to_string();

  let s = "initial contents".to_string();

  let s = String::from("initial contents");

  let hello = String::from("नमस्ते");
  println!("Hello in Hindi: {}", hello);

  let mut s = String::from("foo");
  s.push_str("bar");

  let mut s1 = String::from("foo");
  let s2 = "bar";
  s1.push_str(s2);
  println!("s2 is {}", s2);

  let mut s = String::from("lo");
  s.push('l');

  let s1 = String::from("Hello, ");
  let s2 = String::from("world!");
  println!("s1: {}", s1);
  let s3 = s1 + &s2;
  println!("s2: {}", s2);
  println!("s1 + s2 = {}", s3);

  let s1 = String::from("tic");
  let s2 = String::from("tac");
  let s3 = String::from("toe");

  let s = s1 + "-" + &s2 + "-" + &s3; // s1 value has moved here

  let s1 = String::from("tic");
  let s2 = String::from("tac");
  let s3 = String::from("toe");

  let s = format!("{}-{}-{}", s1, s2, s3);
  println!("s1: {}, s2: {}, s3: {}", s1, s2, s3);

  let hello = "Здравствуйте";
  let s = &hello[0..2];
  println!("first letter of Здравствуйте {}", s);

  for c in hello.chars() {
    println!("{}", c);
  }

  let hello = "नमस्";
  for c in hello.chars() {
    println!("char: {}", c);
  }

  for c in hello.bytes() {
    println!("byte: {}", c);
  }

  // Storing Keys with Associated Values in Hash Maps
  use std::collections::HashMap;

  let mut scores = HashMap::new();

  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 50);

  let mut teams = vec![String::from("Blue"), String::from("Yellow")];
  let mut initial_values = vec![10, 50];

  let scores: HashMap<_, _> = teams.iter().zip(initial_values.iter()).collect();

  for (k, v) in scores.iter() {
    println!("{} team score: {}", k, v);
  }

  let field_name = String::from("Favorite color");
  let field_value = String::from("Blue");

  let mut map = HashMap::new();
  map.insert(field_name, field_value);
  // Accessing field_name or field_value won't work has the ownership is taken by previous function
  // field_value;
  //
  let mut scores = HashMap::new();
  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 50);

  let team_name = String::from("Blue");
  let score = scores.get(&team_name);
  println!("Blue team score: {:?}", score);
  scores.insert(String::from("Blue"), 10);
  println!("{:?}", scores);

  let mut scores = HashMap::new();
  scores.insert(String::from("Blue"), 10);

  scores.entry(String::from("Yellow")).or_insert(50);
  scores.entry(String::from("Blue")).or_insert(50);

  println!("{:?}", scores);

  let text = "hello world wonderful world";

  let mut map = HashMap::new();

  for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
  }

  println!("{:?}", map);

  // uncomment below lines to test panic
  // let v = vec![1, 2, 3];

  // v[99];

  // panic!("crash and burn");
  use std::fs::File;

  let f = File::open("hello.txt");

  let f = match f {
    Ok(file) => file,
    Err(error) => panic!("problem opening the file: {:?}", error),
  };
}
