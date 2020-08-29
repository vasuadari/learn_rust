// Method Syntax
// Learn how to define methods for a struct
#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn square(size: u32) -> Rectangle {
      Rectangle { width: size, height: size }
  }
}

impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }

  fn can_hold(&self, rect2: &Rectangle) -> bool {
    println!("{} > {} width && {} > {} height", self.width, rect2.width, self.height, rect2.height);
    self.width > rect2.width && self.height > rect2.height
  }
}

fn main() {
  let rect1 = Rectangle { width: 30, height: 45 };
  let rect2 = Rectangle { width: 10, height: 40 };
  let rect3 = Rectangle { width: 40, height: 10 };

  println!("rect1 is {:#?}", rect1);

  println!("The area of the rectangle is {} square pixels.",
    rect1.area()
  );

  println!("rect2 is {:#?}", rect2);

  println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

  println!("rect3 is {:#?}", rect3);

  println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

  let square1 = Rectangle::square(10);
  println!("square1 is {:#?}", square1);
}

