use crate::calculate_T_value;

fn initial_rotation(r: u8) -> bool { //TODO: Make it more complex
    return r < 200;
}

//If this algorithm had a name, that would be SYNTOM.
pub fn magic(given_password: &Vec<u8>, input: &Vec<u8>, t: &u8, encryption: bool) -> Vec<u8> {
    let mut cur_pass_byte: usize = 0; //TODO: Check size
    let max_i = given_password.len();
    let mut bit_count = 0;
    let mut iter = 0;

    let mut res_vec: Vec<u8> = Vec::new();
    let mid_nums = (t % 100) / 10;   //(r % 100) / 10 keeps only the "decades", e.g. 3 from 237, 7 from 171, etc
    let last_digit = t % 10;
    let magic_bool = initial_rotation(t.clone());

    let initial_rotation_is_left: bool = if encryption { magic_bool } else { !magic_bool };
    let how_many_left_rots = if encryption { last_digit } else { mid_nums };
    let how_many_right_rots = if encryption { mid_nums } else { last_digit };

    let mut left_rot_counter = 0;
    let mut right_rot_counter = 0;
    let mut next_rotation_left = initial_rotation_is_left;

    println!("r: {}", t);

    input.iter().for_each(|byte| {
        let n: u8 = given_password.get(cur_pass_byte).unwrap() ^ (t >> bit_count); // 240 for 11110000
        let tmp: u8;

        if next_rotation_left {
            tmp = byte.rotate_left(n.count_ones());
            left_rot_counter += 1;
        } else {
            tmp = byte.rotate_right(n.count_ones());
            right_rot_counter += 1;
        }

        if iter == 0 {
            next_rotation_left = !next_rotation_left;
        }

        if right_rot_counter == how_many_right_rots {
            next_rotation_left = true;
            right_rot_counter = 0;
        }

        if left_rot_counter == how_many_left_rots {
            next_rotation_left = false;
            left_rot_counter = 0;
        }

        iter += 1;

        if bit_count == 6 {
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


#[test]
fn it_works() {
    let input = Vec::from("Encrypt me!");
    let pass = Vec::from("123");
    let t = calculate_T_value(&pass);
    let encrypted = magic(&pass, &input, &t, true);
    let decrypted = magic(&pass, &encrypted, &t, false);
    assert_ne!(input, encrypted);
    assert_eq!(input, decrypted);

}