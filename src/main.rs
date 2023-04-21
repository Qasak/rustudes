mod probability;
use itertools::Itertools;

fn main() {
    println!("Hello, rustudes!");
    let v = vec!["R5", "R9", "W1", "W2", "W4", "R7", "B4", "B2", "W3", "R2", "R1", "W7", "R8", "R6", "R4", "B3", "W8", "B6", "R3", "W5", "W6", "B1", "B5"];
    let it = v.iter().combinations(6).collect::<Vec<_>>();
    println!("{:?}", it);
}
