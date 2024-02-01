use std::{env, fs};
use std::fs::File;
use std::io::Read;
use crate::encdec::magic;

mod encdec;
mod main_old;

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

/*
fn initial_rotation(r: u8) -> bool { //TODO: Make it more complex
    return r < 200;
}


fn rotate_right_first(given_password: &Vec<u8>, input: &Vec<u8>, r: &u8, encryption: bool) -> Vec<u8> {
    let mut cur_pass_byte: usize = 0; //TODO: Check size
    let max_i = given_password.len();
    let mut bit_count = 0;
    let mut iter = 0;

    let mut res_vec: Vec<u8> = Vec::new();
    let mid_nums = (r % 100) / 10;   //(r % 100) / 10 keeps only the "decades", e.g. 3 from 237, 7 from 171, etc
    let last_digit = (r % 10);
    let magic_bool = initial_rotation(r.clone());

    let initial_rotation_is_left: bool = if (encryption) { magic_bool } else { !magic_bool };
    let how_many_left_rots = if (encryption) { last_digit } else { mid_nums };
    let how_many_right_rots = if (encryption) { mid_nums } else { last_digit };

    let mut left_rot_counter = 0;
    let mut right_rot_counter = 0;
    let mut next_rotation_left = initial_rotation_is_left;

    println!("r: {}", r);

    input.iter().for_each(|byte| {
        let n: u8 = given_password.get(cur_pass_byte).unwrap() ^ (240 >> bit_count); // 240 for 11110000
        let tmp: u8;

        if next_rotation_left {
            tmp = byte.rotate_left(n.count_ones());
            left_rot_counter += 1;
        } else {
            tmp = byte.rotate_right(n.count_ones());
            right_rot_counter += 1;
        }

        if (iter == 0) {
            next_rotation_left = !next_rotation_left;
        }

        if (right_rot_counter == how_many_right_rots) {
            next_rotation_left = true;
            right_rot_counter = 0;
        }

        if (left_rot_counter == how_many_left_rots) {
            next_rotation_left = false;
            left_rot_counter = 0;
        }

        iter += 1;

        if (bit_count == 6) {
            bit_count = 0;
            cur_pass_byte += 1;
        } else {
            bit_count += 2;
        }
        if (cur_pass_byte == max_i) {
            cur_pass_byte = 0;
        }
        res_vec.push(tmp);
    });

    return res_vec;
}




    args: 1. e h d (encrypt h decrypt)
          2. file path (gia thn wra aplws ena string)
          3. pass
  1. diaxwrizoume to input se octades.
    2. Pairnoume ta bits tou password ana oktades kai ta antimetwpizoume ws integers (dhl. oxi characters genika kai aorista)
        Ta kanoume diadoxika XOR metaksu tous, kai katalhgoume se ena byte pou antistoixei se arithmo apo 000 ews 255.
        To prwto pshfio orizei an tha ksekinhsoume apo aristero (0, 1) h deksi (2) rotation.
        To deutero pshfio orizei, apo to deutero rotation kai meta, posa deksia rotations tha ginontai diadoxika.
        To trito antistoixa gia diadoxika aristera rotations.

        Rotate X (deksia h aristera) th prwth oktada tou input, tosa pshfia osa einai oi assoi tou prwtou byte tou kwdikou XOR 11110000.
        Rotate (Deksia h aristera) th deuterh, tosa pshfia osa einai oi assoi tou deuterou byte XOR 01111000


    3. idia diadikasia apokruptografhshs, me antestrammena ta X, Y



    extras:
    A. otan oloklhrwnetai o prwtos kuklos sto 2o vhma, kwdikos XOR antestrammenos kwdikos gia padding.
        O elegxos tou kwdikou tha ginetai mesw autou prin arxisei h apokruptografhsh.
        l
    B. o kwdikos DEN MENEI STH RAM H STH CACHE PERISSOTERO APO O,TI XREIAZETAI.
        Lave o,ti metra xreiazontai gia auto.


 */