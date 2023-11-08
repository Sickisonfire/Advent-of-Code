pub fn solve() {
    let input = include_str!("../../inputs/three");
    let c = 'Z' as u8;
    let b = 0b1100000;
    let c: u8 = c ^ b;
    let i: usize = c.try_into().unwrap();
    println!("{:b} {}", c, i - 6);
}
