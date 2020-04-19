use std::io;

fn main() {
    println!("Convert fahrenheit to celsius.\nPress CTRL + C to exit.");

    loop {
      println!("Enter degress in fahrenheit");

      let mut fahrenheit = String::new();

      io::stdin().read_line(&mut fahrenheit)
          .expect("failed to read line");

      let fahrenheit: f32 = fahrenheit.trim().parse()
          .expect("entered value is not in decimals");

      println!("Celsuis: {}", to_celsius(fahrenheit));
    }
}

fn to_celsius(f: f32) -> f32 {
    (f - 32.0) * 5.0 / 9.0
}
