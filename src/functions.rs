pub fn run() {
    greeting("Hello", "Adil");
  
    // Bind function values to variables
    let get_sum = add(5, 5);
    println!("Sum: {}", get_sum);
  
    // Closure
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3; //can use colsures outside of vars
    println!("C Sum: {}", add_nums(3, 3));
}
  
fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}


//when returning something ->
fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2 //dont use semicolon because we want to return this
}