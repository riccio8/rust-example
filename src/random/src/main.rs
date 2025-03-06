// The prelude import enables methods we use below, specifically
// Rng::random, Rng::sample, SliceRandom::shuffle and IndexedRandom::choose.
use rand::prelude::*;
use std::io;


fn main(){
    // Get an RNG:
    let mut rng = rand::rng();

    println!("Write a num from 0 to 100");

    let nums: Vec<i32> = (0..100).collect();

    let pc_choice: Option<&i32> = nums.choose(&mut rng);
    if pc_choice.is_none() {
        println!("Error");
        return;
    }
    
    let mut user_cho: String = String::new();

    io::stdin().read_line(&mut user_cho).unwrap();

    
    let n = match user_cho.trim().parse::<i32>(){
        Ok(res) => res,
        Err(_) => {
            println!("error while converting into int32");
            return;
        }
    };


    if pc_choice == Some(&n) {
        println!("u win");
    }else{
        println!("u loose");
    }
}

