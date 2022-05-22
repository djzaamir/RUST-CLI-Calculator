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

    //Now the iterator as advanced forward, and the next valid values
    // is at index == 0, so we will unwrap index 0 
    let operator = cli_args.nth(0).unwrap();

    //Again getting the data from index == 0
    let second_value = cli_args.nth(0).unwrap();
}
