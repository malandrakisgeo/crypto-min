use std::{env, fs};
use std::fs::File;
use std::io::Read;
use crate::encdec::magic;

mod encdec;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        panic!("Wrong number of args!")
    }
    let end_or_dec = args[1].clone(); // -e gia encrypt
    // let file_or_str = &args[2]; //-f gia arxeio

    let input = args[2].clone();
 //   let octades = Vec::from(input);
    let metadata = fs::metadata("/home/georgem/RustroverProjects/crypto-min/target/debug/primenumbers").expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
     File::open("/home/georgem/RustroverProjects/crypto-min/target/debug/primenumbers").unwrap().read(&mut *buffer).expect("TODO: panic message");
    let octades = buffer.to_vec();

    let pass = args[4].clone();
    println!("p {}", pass);
    let v = Vec::from(pass);
    let t = calculate_T_value(&v);
    let bv = magic(&v, &octades, &t, true);
    let p = magic(&v, &bv, &t, false);
    let rr = p.len();

    println!("{:?}", String::from_utf8(octades));
    println!("{:?}", String::from_utf8(p));
}

// T = first bit of password XOR last, if the password has an odd number of characters. Otherwise T = first_bit XNOR last_bit.
fn calculate_T_value(given_password: &Vec<u8>) -> u8 {
    let r = given_password.len();
    let mut t: u8 = 0;


    given_password.iter().for_each(|byte| {
        if(given_password.len() % 2 == 0){
            t = t ^ byte;
        } else{
            t = !(t ^ byte);

        }
        println!("{:08b}", byte);
    });

    return t;
}

