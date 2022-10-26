use rand::Rng;

fn main() {
    const MAX_CELL: usize = 64;
    let cells: [[i32; MAX_CELL / 2]; MAX_CELL / 2] = Default::default();
    let mut rng = rand::thread_rng();

    for _j in 0..cells.len() {
        for _i in 0..cells.len() {
            let rand = rng.gen_range(0..2);
            if rand == 0 {
                print!("{}", "■");
            }
            else {
                print!("{}", "□");
            }
        }
        println!();
    }
}
