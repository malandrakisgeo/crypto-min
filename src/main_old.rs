use std::env;


fn mainn() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5{
        panic!("Wrong number of args!")
    }
    let end_or_dec = args[1].clone(); // -e gia encrypt
   // let file_or_str = &args[2]; //-f gia arxeio

    let input = args[2].clone();
    let octades = Vec::from(input);


    let pass = args[4].clone();
    println!("p {}", pass);
    let v = Vec::from(pass);
    rotate_right_first(&v, &octades);

}
// T = first bit of password XOR last, if the password has an odd number of characters. Otherwise T = first_bit XNOR last_bit.
fn calculate_T_value(given_password: &Vec<u8>) -> bool{
    let t: bool;
    println!("{:08b}", given_password.get(0).unwrap());
    println!("{:08b}", given_password.get(given_password.len()-1).unwrap());

    let first_bit: u8 = (given_password.get(0).unwrap()) & 128; //binary_value AND 10000000
    let last_bit: u8 = (given_password.get(given_password.len()-1).unwrap() ) & 1; //binary_value AND 00000001
    let tmp = (first_bit ^ last_bit);
    let res: u8;

    if (given_password.len() % 2) == 0{
        res = !tmp;
    }else{
        res =  tmp;
    }

    if(res == 0){
        return false;
    }

    return true;
}

fn rotate_right_first(given_password: &Vec<u8>, input: &Vec<u8>){ //t == true
     let mut i: usize = 0; //TODO: Check size
    let max_i = given_password.len();
    let mut bit_count = 0;

    let mut res_vec: Vec<u8> = Vec::new();
    let mut iteration: usize = 0;

    input.iter().for_each(|byte| {

        let n: u8 = given_password.get(i).unwrap() & (240 >> bit_count); // 240 for 11110000, 192 for 11000000
        let tmp: u8;
        println!("byte before rotation {:08b}",byte);
        println!(" 192 > bit_count: {}",192 >> bit_count);
        println!("rotation by: {}",n);
        println!("rotation by: {:08b}",n);

        if (iteration % 2 == 0){
            tmp = byte.rotate_right(n.count_ones());
        }else{
            tmp = byte.rotate_left(n.count_ones());
        }
        iteration +=1;
        println!("after rotation: {:08b} ",  tmp);


        println!("{}",i);
        println!(" bit_count: {}", bit_count);


        if (bit_count == 6){
            bit_count = 0;
            i+=1;
        }else{
            bit_count+=2;
        }
        if (i == max_i){
            i = 0;
        }


        res_vec.push(tmp);
    })
}


/*

    args: 1. e h d (encrypt h decrypt)
          2. file path (gia thn wra aplws ena string)
          3. pass

    1. diaxwrizoume to input se octades.
    2.
        rotate X th prwth oso ta duo prwta bits tou kwdikou an upologistoun ws dekadikos (dhl. apo 0 ews 3)
        rotate Y th deuterh oso ta duo deutera
        rotate X th 3h oso to 5o kai 6o bits tou kwdikou
        k.o.k
        assert_eq!(n.rotate_left(8), m);

       X=deksia h aristera, analoga to an to T einai mhden h ena antistoixa.

        T: an o arithmos twn xarakthrwn einai monos, T = prwto bit XOR teleutaio. Eidallws XNOR.

    3. idia diadikasia apokruptografhshs, me antestramena ta X, Y


    extras:
    A. otan oloklhrwnetai o prwtos kuklos sto 2o vhma, kwdikos XOR antestrammenos kwdikos gia padding.
        O elegxos tou kwdikou tha ginetai mesw autou prin arxisei h apokruptografhsh.
        l
    B. o kwdikos DEN MENEI STH RAM H STH CACHE PERISSOTERO APO O,TI XREIAZETAI.
        Lave o,ti metra xreiazontai gia auto.


 */