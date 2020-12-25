
pub fn part_one() {
    println!("--- Part One ---");

    let pub1 = 1327981u64;
    let pub2 = 2822615u64;
    let modulus = 20201227u64;

    let  subject = 7u64;

    let mut loop_size_1 = 1;
    let mut loop_size_2 = 1;

    let mut current = 1;
    loop {
        current = (current * subject) % modulus;
        if current == pub1 {
            break;
        }
        loop_size_1 += 1;
    }

    current = 1;
    loop {
        current = (current * subject) % modulus;
        if current == pub2 {
            break;
        }
        loop_size_2 += 1;
    }

    current = 1;
    for i in 0..loop_size_2 as usize {
        current = (current * pub1) % modulus;
    }

    println!("Result: {:?}", current);
}

pub fn part_two() {
    println!("--- Part One ---");

    println!("Result: {:?}", 0);
}
