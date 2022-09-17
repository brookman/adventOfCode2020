mod contest;
mod common;
mod domain;
mod serialization;

fn main() {
    println!("\nCoding contest\n");
    contest::run("./data/inputA.txt","./data/outputA.txt");
    contest::run("./data/inputB.txt","./data/outputB.txt");
    contest::run("./data/inputC.txt","./data/outputC.txt");
    contest::run("./data/inputD.txt","./data/outputD.txt");
    contest::run("./data/inputE.txt","./data/outputE.txt");
}
