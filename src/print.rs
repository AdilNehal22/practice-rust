


pub fn run() {
    println!("hello");
    //formatting
    println!("no is: {}", 1);
    println!("{} is my {}", "Adil", "name");

    //positional arguments
    println!(
        "{0} is my {1} and {0} is {2} developer",
        "Adil", "name", "blockchain"
    );

    //named arguments
    println!(
        "{name} is a {lovesWhat}",
        name = "Adil",
        lovesWhat = "Monarchist"
    );

    //placeholder traits
    println!("binary: {:b}, hex: {:x}, octal: {:0}", 10, 10, 10);

    //placeholder for debug traits
    println!("{:?}", (12, true, "hello"));
}