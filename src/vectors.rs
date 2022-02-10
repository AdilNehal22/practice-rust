//vectors are resizable arrays

pub fn run(){

    let mut arrayOfNumbersSize5: Vec<i32> = vec![1,2,3,4,5];
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

    
    //reassiging
    arrayOfNumbersSize5.push(6);

    //pop the last value
    arrayOfNumbersSize5.pop();

    for x in arrayOfNumbersSize5.iter(){
        println!("numbers {}", x);
    }

    //looping each value and mutate them
    for y in arrayOfNumbersSize5.iter_mut(){
        *y *= 2;
    }

    println!("numbers {:?}", arrayOfNumbersSize5);







}   