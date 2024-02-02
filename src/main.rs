use std::{env, fs};
use std::fs::File;
use std::io::{Read, Write};
use crate::encdec::magic;

mod encdec;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        panic!("Wrong number of args!")
    }
    let enc = args[1].clone();
    let path = args[2].clone();
    let pass = args[4].clone();
    let encrypt: bool = enc.eq("-e");

    let cr_f_name = if (encrypt) { path.clone() + ".cr.min"} else { path.clone() + "dec"};

 //   let octades = Vec::from(input);
    let metadata = fs::metadata(path.clone()).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
     File::open(path.clone()).unwrap().read(&mut *buffer).expect("TODO: panic message");
    let octades = buffer.to_vec();


    let passvec = Vec::from(pass);
    let t = calculate_T_value(&passvec);
    let result_vector = magic(&passvec, &octades, &t, encrypt);

    let mut f = File::create(cr_f_name).unwrap();
    f.write(&result_vector).expect("TODO: panic message");


}

// T = first bit of password XOR last, if the password has an odd number of characters. Otherwise T = first_bit XNOR last_bit.
pub fn calculate_T_value(given_password: &Vec<u8>) -> u8 {
    let len = given_password.len();
    let mut t: u8 = 0;


    given_password.iter().for_each(|byte| {
        if(len % 2 == 0){
            t = t ^ byte;
        } else{
            t = !(t ^ byte);

        }
        println!("{:08b}", byte);
    });

    return t;
}

