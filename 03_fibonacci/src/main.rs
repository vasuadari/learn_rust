use std::io;

fn main() {
    println!("No. of fibonacci numbers to print!");

    let mut number = String::new();

    io::stdin().read_line(&mut number)
        .expect("failed to read line");

    let n: i32 = number.trim().parse()
        .expect("invalid number");

    // `a` will be printed
    // `b` will store next value
    // `c` will be used to store sum of a & b and moved to b.
    //     Its a temporary variable.
    let mut a = 0;
    let mut b = 1;
    let mut c = 0;

    for _i in c..n {
        // uncomment to debug
        // println!("a: {} b: {} c: {}", a, b, c);
        println!("{}", a);
        c = a + b;
        a = b;
        b = c;
    }
}

