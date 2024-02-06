use std::io::Read;
use rand::random;

fn determine_padding_positions(given_password: &Vec<u8>, input_length: usize, padding_length: usize) -> Vec<usize> {
    let mut positions: Vec<usize> = Vec::new();

    let mut i = 0;

    let mut curr_pos: usize = 1;
    let mut password_pointer = 0;

    while i < padding_length {
        if curr_pos < input_length {
            if password_pointer == given_password.len() {
                password_pointer = 0;
            }
            let next_pos: usize = curr_pos + (usize::from(given_password.get(password_pointer).unwrap() & 7) ); //it was +1
            if next_pos <= input_length{
                positions.push(next_pos);
            }
            curr_pos = next_pos;
            password_pointer += 1;

        }else{
            if i <= curr_pos{
               // let d =  curr_pos +  1;
                i += curr_pos;
            }
            break;
        }
        i += 1;
    }
    positions
}



fn get_combined_vector(padding_bytes: &Vec<u8>, positions: &Vec<usize>, input: &Vec<u8>) -> Vec<u8> {
    let mut padding_pointer = 0;
    let mut input_pointer = 0;
    let mut total_vec: Vec<u8> = Vec::new();
    let mut i: usize = 0;

    while i < (input.len() + padding_bytes.len()) { //TODO: Ftiakse auto to aisxos
        if input_pointer==input.len(){
            let rest = &padding_bytes.as_slice()[padding_pointer..];
            total_vec.append(&mut Vec::from(rest));
            break;
        }
        if padding_pointer>= padding_bytes.len() {
            let rest_input = &input.as_slice()[input_pointer..];
            total_vec.append(&mut Vec::from(rest_input));

            let rest = &padding_bytes.as_slice()[padding_pointer..];
            total_vec.append(&mut Vec::from(rest));
            break;
        }
        if positions.contains(&i) {
                total_vec.push(padding_bytes.get(padding_pointer).unwrap().clone());
                padding_pointer += 1;

        } else {
            if input.len() > input_pointer {
                total_vec.push(input.get(input_pointer).unwrap().clone());
                input_pointer += 1;}
        }
        i += 1;
    }
    total_vec
}
fn determine_padding_size(input: usize) -> usize {
    let pad_s: usize;
    if input <= 1024 {
        pad_s = 256;
    } else if input > 1024 && input <= 10240{
        pad_s = 512;
    }else if input > 10240 && input <= 1024000{
        pad_s = 2048;
    }else if input > 1024000 && input <= 102400000{
        pad_s = 4096;
    }else{
        pad_s = 8192;
    }
    /*else {
        let d = ((input / 1024) * 64) ; //adds 6.25 % overhead
        pad_s = d; //d + (d % 1024); //TODO: Use this and find a way of reversing it without losing a bit.
    }*/

    return pad_s;
}

fn determine_original_size(total_input: usize, password_length: usize) -> usize {
    let pad_s: usize;
    let input = total_input - 2*password_length;
    if input <= 1024+256 {
        pad_s = input - 256;
    } else if input > 1024+512 && input <= 10240+512{
        pad_s = input - 512;
    }else if input > 10240+2048 && input <= 1024000+2048{
        pad_s = input - 2048;
    }else if input > 1024000+4096 && input <= 102400000+4096{
        pad_s = input - 4096;
    }else{
        pad_s = input - 8192;
    }

    return pad_s;
}

pub fn add_padding(given_password: &Vec<u8>, input: &Vec<u8>) -> Vec<u8> {
    let N = determine_padding_size(input.len());
    let mut padding_bytes: Vec<u8> = Vec::new();
    let mut to_be_returned: Vec<u8> = Vec::new();

    let mut positions: Vec<usize> = Vec::new();
    let mut i = 0;

    while i < N {
        let random_number: u8 = random();
        padding_bytes.push(random_number);
        i += 1;
    }
    i = 0;

    positions = determine_padding_positions(&given_password, input.len(), N);

    let mut total_vec = get_combined_vector(&padding_bytes, &positions, &input);
    let mut pass_padd: Vec<u8> = Vec::new();
    let mut pass_padd_end: Vec<u8> = Vec::new(); //if the padding is the same in the beginning and at the end, we give attackers the password length. 

    while i < given_password.len() {
        let mut random_number: u8 = random();
        pass_padd.push(random_number);
        random_number = random();
        pass_padd_end.push(random_number);
        i += 1;
    }

    to_be_returned.append(&mut pass_padd);

    to_be_returned.append(&mut total_vec);

    to_be_returned.append(&mut pass_padd_end);

    return to_be_returned;
}

pub fn remove_padding(given_password: &Vec<u8>, input: &Vec<u8>) -> Vec<u8> {
    let mut to_be_returned: Vec<u8> = Vec::new();
    let mut positions: Vec<usize> = Vec::new();
    let mut i = 0;
    let pass_length =  given_password.len();
    let interesting = Vec::from(&input.as_slice()[pass_length..]);

    let N = determine_original_size(input.len(), pass_length);
    let pagging_length = input.len() - N - 2*pass_length;

    positions = determine_padding_positions(&given_password, N, pagging_length);
    let last_pos = positions.get(positions.len() -1 ).unwrap().clone();
    while i<last_pos {
        if !positions.contains(&i) {
            to_be_returned.push(interesting.get(i).unwrap().clone());
        }
        i += 1;

    }
    let mut l = last_pos +1;
    while l<N+positions.len(){
        to_be_returned.push(interesting.get(l).unwrap().clone());
        l += 1;
    }


    return to_be_returned;
}

#[test]
fn padding() {
    let input = Vec::from("gooby pls"); //Encrypt me!
    let pass = Vec::from("4"); //147
    let t = crate::calculate_T_value(&pass);
    let encrypted = crate::magic(&pass, &input, &t, true);
    let poss: Vec<usize> = determine_padding_positions(&pass, input.len(), determine_padding_size(input.len()));

    let res = add_padding(&pass, &encrypted);
    assert_eq!(res.len(), 256 + 2*pass.len() + input.len());
    let ress = remove_padding(&pass, &res);
    assert_eq!(ress.len(), input.len());
}


/*
#[test]
fn paddingfile() {
    let metadata = std::fs::metadata("im.png").expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];

     std::fs::File::open("im.png").unwrap().read(&mut *buffer).expect("TODO: panic message");
    let mut input = buffer.to_vec();
    let pass = Vec::from("124"); //t23fgfff
    let t = crate::calculate_T_value(&pass);
    let encrypted = crate::magic(&pass, &input, &t, true);

    let poss: Vec<usize> = determine_padding_positions(&pass, input.len(), determine_padding_size(input.len()));

    let res = add_padding(&pass, &encrypted);
    //let ress = remove_padding(&pass, &res);
    let N = determine_original_size(res.len(), pass.len());
    assert_eq!(N, input.len());
    let positions = determine_padding_positions(&pass, N, res.len() - N - pass.len());
    assert_eq!(positions.len(), poss.len());
    assert_eq!(positions, poss);

}
*/
#[test]
fn check_positions() {
    let pass = Vec::from("222");
    let positions: Vec<usize> = determine_padding_positions(&pass, 10, 10);
    let R1 = usize::from(pass.get(0).unwrap() & 7); //byte AND 00000111
    let P1 = R1 + 1;

    assert_eq!(3, P1);
    assert_eq!(positions.get(0).unwrap().clone(), P1);
}


