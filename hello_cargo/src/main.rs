fn main() {
    let a = [1, 2, 3, 4, 5];
    println!("1st element {}", a[0]);
    println!("length of array {}", a.len());
    print_label();

    let number = 3;

    if number != 0 {
        println!("number was three");
    }

    // loop {
    //     println!("again!");
    // }
}

fn print_label() {
    println!("Hello, world Print");
}
