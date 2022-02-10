

pub fn run(){

    let arrayOfNumbersSize5: [i32; 5] = [1,2,3,4,5];
    println!("{:?}", arrayOfNumbersSize5);
    //getting the single
    println!("{}", arrayOfNumbersSize5[0]);

    //mutable
    let mut mutArrayOfNumbersSize5: [i32; 5] = [1,2,3,4,5];
    mutArrayOfNumbersSize5[3] = 5;

    //get array length
    mutArrayOfNumbersSize5.len();

    //get slice
    let slice: &[i32] = &mutArrayOfNumbersSize5[1..2];

    println!("slice {:?}", mutArrayOfNumbersSize5);

    









}   



