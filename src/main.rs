#![allow(dead_code)]

mod contest;
mod common;
mod vectors;

fn main() {
    println!("\nCoding contest");
    contest::run("./data/inputA.txt","./data/outputA.txt");
    contest::run("./data/inputB.txt","./data/outputB.txt");
    contest::run("./data/inputC.txt","./data/outputC.txt");
    contest::run("./data/inputD.txt","./data/outputD.txt");
    contest::run("./data/inputE.txt","./data/outputE.txt");
}
