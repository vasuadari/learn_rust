use std::io;

fn main() {
    println!("No. of fibonacci numberc to print!");

    let mut number = String::new();

    io::stdin().read_line(&mut number)
        .expect("failed to read line");

    let n: i32 = number.trim().parse()
        .expect("invalid number");

    let mut a = 0;
    let mut b = 1;
    let mut c = 0;

    for _i in c..n {
        println!("{}", a);
        c = a + b;
        a = b;
        b = c;
    }
}

