// Tuples group together values of different types
// Max 12 elements


pub fn run(){

    let person: (&str, &str, i8) = ("Adil", "Blockchain", 10);


    println!("{} is {} and {}", person.0, person.1, person.2);


}