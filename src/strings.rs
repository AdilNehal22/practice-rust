//primitive string = immutable fixed length string somewhere in memory
//String Growble = heap allocated data strucuture


pub fn run(){

    let hello = "Adil"; //primitive
    let mut adil = String::from("hello"); //growable

    let hello_len = hello.len(); //will give length of string of both type

    //growing the string of second type
    adil.push('n'); //push only a char
    adil.push_str("ehal"); //pushing a whole string

    //hello.is_empty();

    //if contains?
    adil.contains("Nehal");

    //replaces
    hello.replace("Adil", "Nehal");

    //loop through string with whitespace

    for word in hello.split_whitespace(){
        println!("word{}", word);
    }

    //create string with capacity
    let mut s = String::with_capacity(10);
    s.push_str("Hello");

    //assertion testing, only show on console when assertion fails
    assert_eq!(2, s.len());


    println!("{:?}", (hello, adil, hello_len));



    
}