fn main() {
    println!("{} days", 31);

    // positional arguments
    println!("{0} is not {1}, and {1} is not {0}", "Alice", "Bob");

    // named arguments
    println!(
        "{subject} {verb} {object}",
        object = "burger",
        verb = "eats",
        subject = "Greg"
    );

    // special formatting with ':', in this case binary
    println!(
        "{} of {:b} people know binary, the other half doesn't",
        1, 2
    );

    // prefix with 6 whitespaces
    println!("{number:>width$}", number = 1, width = 6);

    // prefix with 6 0s
    println!("{number:>0width$}", number = 1, width = 6);

    // print decimal point with precision of 3
    let pi = 3.141592;
    println!("Pi is roughly {0:.1$}", pi, 3);
}
