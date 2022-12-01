mod days;
use days::one;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).take(1).collect();

    match args[0].parse::<usize>() {
        Ok(day) => match day {
            1 => one::solve(),
            2 => unimplemented!(),
            3 => unimplemented!(),
            4 => unimplemented!(),
            5 => unimplemented!(),
            6 => unimplemented!(),
            7 => unimplemented!(),
            8 => unimplemented!(),
            9 => unimplemented!(),
            10 => unimplemented!(),
            11 => unimplemented!(),
            12 => unimplemented!(),
            13 => unimplemented!(),
            14 => unimplemented!(),
            15 => unimplemented!(),
            16 => unimplemented!(),
            17 => unimplemented!(),
            18 => unimplemented!(),
            19 => unimplemented!(),
            20 => unimplemented!(),
            21 => unimplemented!(),
            22 => unimplemented!(),
            23 => unimplemented!(),
            24 => unimplemented!(),
            25 => unimplemented!(),
            _ => panic!("not a day"),
        },
        Err(e) => panic!("something wrong: {}", e),
    };
}
