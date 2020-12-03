mod dec_01;
mod dec_02;
mod dec_03;
mod common;

fn main() {
    println!("\nDecember 1st, 2020");
    dec_01::part_one();
    dec_01::part_two();

    println!("\nDecember 2nd, 2020");
    dec_02::part_one();
    dec_02::part_two();

    println!("\nDecember 3rd, 2020");
    dec_03::part_one();
    dec_03::part_two();
}
