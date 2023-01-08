use rand::Rng;

const GENERATION: usize = 50;

const RULE_MAX_CELL: usize = 3;
const BIRTH_PATTERN: usize = RULE_MAX_CELL;
const ALIVE_PATTERN: usize = RULE_MAX_CELL + 1;
const DEATH_PATTERN1: usize = RULE_MAX_CELL;
const DEATH_PATTERN2: usize = RULE_MAX_CELL;

const MAX_CELL: usize = 64;
const ALIVE_CELL: &str = "■";
const DEAD_CELL: &str = "□"; const BIRTH_ARRAY: [[usize; BIRTH_PATTERN]; BIRTH_PATTERN] = [[1, 1, 0], [1, 0, 0], [0, 0, 0]]; const ALIVE_ARRAY: [[usize; ALIVE_PATTERN]; ALIVE_PATTERN] = [[0, 0, 0, 0], [0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0]];
const DEATH_ARRAY1: [[usize; DEATH_PATTERN1]; DEATH_PATTERN1] = [[0, 0, 0], [0, 1, 1], [0, 0, 0]];
const DEATH_ARRAY2: [[usize; DEATH_PATTERN2]; DEATH_PATTERN2] = [[1, 1, 1], [1, 1, 0], [0, 0, 0]];

struct LifeGameRule {
    birth: [[usize; BIRTH_PATTERN]; BIRTH_PATTERN],
    alive: [[usize; ALIVE_PATTERN]; ALIVE_PATTERN],
    death1: [[usize; DEATH_PATTERN1]; DEATH_PATTERN1],
    death2: [[usize; DEATH_PATTERN2]; DEATH_PATTERN2],
}

enum Rule {
    Birth,
    Alive,
    Death,
}

fn birth(p: [[usize; BIRTH_PATTERN]; BIRTH_PATTERN]) -> bool {
    if p == BIRTH_ARRAY {
        return true;
    } else {
        return false;
    }
}

fn alive(p: [[usize; ALIVE_PATTERN]; ALIVE_PATTERN]) -> bool {
    if p == ALIVE_ARRAY {
        return true;
    } else {
        return false;
    }
}

fn death1(p: [[usize; DEATH_PATTERN1]; DEATH_PATTERN1]) -> bool {
    if p == DEATH_ARRAY1 {
        return true;
    } else {
        return false;
    }
}

fn death2(p: [[usize; DEATH_PATTERN2]; DEATH_PATTERN2]) -> bool {
    if p == DEATH_ARRAY2 {
        return true;
    } else {
        return false;
}

fn create_cell(i: usize) -> String {
    let result;

    match i {
        0 => result = ALIVE_CELL,
        1 => result = DEAD_CELL,
        _ => result = DEAD_CELL,
    }

    return result.to_string();
}

fn search_cell (cell: [[usize; MAX_CELL / 2]; MAX_CELL / 2], j: usize, i: usize) -> bool {
    println!(" @{}@,  @{}@", j, i);
    return true;
}

fn main() {
    let cells: [[usize; MAX_CELL / 2]; MAX_CELL / 2] = [[0; MAX_CELL / 2]; MAX_CELL / 2];
    let mut new_cells: [[usize; MAX_CELL / 2]; MAX_CELL / 2] = Default::default();
    let mut rng = rand::thread_rng();

    let rule = LifeGameRule {
        birth: [[1, 1, 0], [1, 0, 0], [0, 0, 0]],
        alive: [[0, 0, 0, 0], [0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0]],
        death1: [[0, 0, 0], [0, 1, 1], [0, 0, 0]],
        death2: [[1, 1, 1], [1, 1, 0], [0, 0, 0]],
    };

    // UPDATE: NEW CELLS
    for n in 0..GENERATION {
        println!("GENERATION : _{}_", n + 1);
        for _j in 0..cells.len() {
            for _i in 0..cells.len() {
                let rand = rng.gen_range(0..2);
                    let is_alive = search_cell(cells, _j, _i);

                print!("{}", create_cell(rand))
            }
            println!();
        }
        let mut _input_string = String::new();
        std::io::stdin().read_line(&mut _input_string).ok();
    }
}
