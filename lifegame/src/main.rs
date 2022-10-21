
fn main() {
    let even = "偶数";
    let odd  = "奇数";
    let cells: [i32; 32] = Default::default();
    // let cells: [i32; 32] = [1: 33];

    for n in 1..cells.len() {
        if n % 2 == 0 {
            println!("{} : {}", n, even);
        }
        else {
            println!("{} : {}", n, odd);
        }
    }
}