use rand::Rng;

const BIRTH_PATTERN: usize = 3;
const ALIVE_PATTERN: usize = 4;
const DEATH_PATTERN1: usize = 3;
const DEATH_PATTERN2: usize = 3;

const MAX_CELL: usize = 64;
const ALIVE_CELL: &str = "■";
const DEAD_CELL: &str = "□";

struct RuleStruct {
    birth: [[usize; BIRTH_PATTERN]; BIRTH_PATTERN],
    alive: [[usize; ALIVE_PATTERN]; ALIVE_PATTERN],
    death1: [[usize; DEATH_PATTERN1]; DEATH_PATTERN1],
    death2: [[usize; DEATH_PATTERN2]; DEATH_PATTERN2],
}

// enum Rule {
//     Birth,
//     Alive,
//     Death
// }
// 
// fn birth(i: i32) -> Rule {
//     const PATTERN: usize = 3;
//     // let cells: [[i32; PATTERN]; PATTERN] = [[1, 1, 0]; 1, 0, 0]
//     let cells: [i32; PATTERN] = [1, 1, 0];
// 
//     return Rule::Birth
// }

fn create_cell(i: i32) -> String {
    let result;

    match i {
        0 => result = ALIVE_CELL,
        1 => result = DEAD_CELL,
        _ => result = DEAD_CELL,
    }

    return result.to_string();
}

fn main() {
    let cells: [[i32; MAX_CELL / 2]; MAX_CELL / 2] = Default::default();
    let mut rng = rand::thread_rng();


    let rule = RuleStruct {
        birth: [[1, 1, 0], [1, 0, 0], [0, 0, 0]],
        alive: [[0, 0, 0, 0], [0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0]],
        death1: [[0, 0, 0], [0, 1, 1], [0, 0, 0]],
        death2: [[1, 1, 1], [1, 1, 0], [0, 0, 0]],
    };


    for _j in 0..cells.len() {
        for _i in 0..cells.len() {
            let rand = rng.gen_range(0..2);

            print!("{}", create_cell(rand))
        }
        println!();
    }
}
