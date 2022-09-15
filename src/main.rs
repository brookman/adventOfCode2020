#![allow(dead_code)]

mod contest;
mod common;
mod vectors;

fn main() {
    println!("\nCoding contest");
    contest::run("./data/inputA.txt","./outputA.txt");
    // contest::run("./data/inputB.txt","./outputB.txt");
    // contest::run("./data/inputC.txt","./outputC.txt");
    // contest::run("./data/inputD.txt","./outputD.txt");
    // contest::run("./data/inputE.txt","./outputE.txt");
}
