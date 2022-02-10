/**
 * variables stores primitive data or ref to data
 * immutable by default in rust
 * rust is block-scaped language
 * if you want to make a variable mutable use 'mut'
 */

pub fn run(){
    //mutable vars
    let name = "Adil";
    let mut age = 21;
    age = 22;
    println!("my name is {} and age is {}", name, age);
    //const vars, you have to explicitely define type
    const ID: i32 = 001;
    println!("ID is {}", ID);
    //assign multiple variables
    let (my_name, my_age) = ("Adil", 21);
    println!("my name is {} my age {}", my_name, my_age);
}