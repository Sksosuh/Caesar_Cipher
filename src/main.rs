mod cipher;
use cipher::translate;
use std::{fs, io};

fn main() {
    //Turns contents of text file into a tring, return "Should've" if the process fails
    let contents = fs::read_to_string("XenobladeScriptE.txt").expect("Shoudlve");

    println!("encrypt or decrypt?");
    let mut mode = String::new();
    io::stdin().read_line(&mut mode).unwrap();
    println!("Enter Encryption Key:");
    let mut key = String::new();
    io::stdin().read_line(&mut key).unwrap();
    let num: isize = key.trim().parse().unwrap();
    let result = translate(contents, mode, num);
    println!("{}", result);
}
