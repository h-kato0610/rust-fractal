use rand::Rng;

fn create_cell(i: i32) -> String {
    const ALIVE_CELL: &str = "■";
    const DEAD_CELL: &str = "□";
    let result;

    match i {
        0 => result = ALIVE_CELL,
        1 => result = DEAD_CELL,
        _ => result = DEAD_CELL,
    }

    return result.to_string();
}

fn main() {
    const MAX_CELL: usize = 64;
    let cells: [[i32; MAX_CELL / 2]; MAX_CELL / 2] = Default::default();
    let mut rng = rand::thread_rng();

    for _j in 0..cells.len() {
        for _i in 0..cells.len() {
            let rand = rng.gen_range(0..2);

            print!("{}", create_cell(rand))
        }
        println!();
    }
}
