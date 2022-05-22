/*
CODE inspired from FreeCodeCamp video on RUST
URL = https://www.youtube.com/watch?v=MsocPEZBd-M&ab_channel=freeCodeCamp.org
*/

use std::env::{args, Args};

fn main() {

    //Args returns an iterator
    //This means, as soon as you extract the desired value
    //The iterator moves to the next line
    //Lets see it in action
    //We will first extract value from index 1 (First useful argument for us)
    //As index 0 points to the path of the binary
    //From there onwards we will extract values from index 0
    //As the iterator advances, we will extract values from index 0
    //We are declaring cli_args as mutable, as Args struct expects a mutable ref
    //Finally, we also have to unwrap the iterator to get out the data
    let mut cli_args : Args = args();

    //get the first value from index == 1, as index 0 is not useful for us
    let first_value = cli_args.nth(1).unwrap();

    //parse to float
    let first_number = first_value.parse::<f32>().unwrap();


    //Now the iterator as advanced forward, and the next valid values
    // is at index == 0, so we will unwrap index 0 
    let operator = cli_args.nth(0).unwrap();

    //parse to char
    let operator_char = operator.chars().next().unwrap();

    
    //Again getting the data from index == 0
    let second_value = cli_args.nth(0).unwrap();
    
    // parse to float
    let second_number = second_value.parse::<f32>().unwrap();




    let result : f32 = compute(operator_char, first_number, second_number);

    println!("{:?}" , output(first_number, operator_char, second_number, result));

}

fn compute(operator : char, first_value : f32, second_value : f32) -> f32{

    //Using RUST pattern matching to simplify branch statements
    match operator{
        '+' => first_value + second_value,
        '-' => first_value - second_value,
        '/' => first_value / second_value,
        '*'|'x'|'X' => first_value * second_value,
        _ => 0.0
    }
}

fn output(first_number : f32, operator_char : char, second_number : f32, result : f32) -> String{
    format!("{} {} {} = {}", first_number, operator_char, second_number, result)
}