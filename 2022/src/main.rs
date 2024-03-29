mod days;
use days::{five, four, one, seven, six, three, two};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).take(1).collect();

    match args[0].parse::<usize>() {
        Ok(day) => match day {
            1 => one::solve(),
            2 => two::solve(),
            3 => three::solve(),
            4 => four::solve(),
            5 => five::solve(),
            6 => six::solve(),
            7 => seven::solve(),
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
