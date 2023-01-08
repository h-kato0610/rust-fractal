use rand::Rng;

const GENERATION: usize = 1;

const RULE_MAX_CELL: usize = 3;
const BIRTH_PATTERN: usize = RULE_MAX_CELL;
const ALIVE_PATTERN: usize = RULE_MAX_CELL + 1;
const DEATH_PATTERN1: usize = RULE_MAX_CELL;
const DEATH_PATTERN2: usize = RULE_MAX_CELL;

const MAX_CELL: usize = 32;

const ALIVE_NUM: usize = 1;
const DEAD_NUM: usize = 0;
const ALIVE_CELL: &str = "■";
const DEAD_CELL: &str = "□";

const BIRTH_ARRAY: [[usize; BIRTH_PATTERN]; BIRTH_PATTERN] = [[ALIVE_NUM, ALIVE_NUM, DEAD_NUM], [ALIVE_NUM, DEAD_NUM, DEAD_NUM], [DEAD_NUM, DEAD_NUM, DEAD_NUM]];

const ALIVE_ARRAY: [[usize; ALIVE_PATTERN]; ALIVE_PATTERN] = [[DEAD_NUM, DEAD_NUM, DEAD_NUM, DEAD_NUM], [DEAD_NUM, ALIVE_NUM, ALIVE_NUM, DEAD_NUM], [DEAD_NUM, ALIVE_NUM, ALIVE_NUM, DEAD_NUM], [DEAD_NUM, DEAD_NUM, DEAD_NUM, DEAD_NUM]];

const DEATH_ARRAY1: [[usize; DEATH_PATTERN1]; DEATH_PATTERN1] = [[DEAD_NUM, DEAD_NUM, DEAD_NUM], [DEAD_NUM, ALIVE_NUM, ALIVE_NUM], [DEAD_NUM, DEAD_NUM, DEAD_NUM]];

const DEATH_ARRAY2: [[usize; DEATH_PATTERN2]; DEATH_PATTERN2] = [[ALIVE_NUM, ALIVE_NUM, ALIVE_NUM], [ALIVE_NUM, ALIVE_NUM, DEAD_NUM], [DEAD_NUM, DEAD_NUM, DEAD_NUM]];

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
}

fn convert_num_to_string(i: usize) -> String {
    let result;

    match i {
        DEAD_NUM => result = ALIVE_CELL,
        ALIVE_NUM => result = DEAD_CELL,
        _ => result = DEAD_CELL,
    }

    return result.to_string();
}

fn search_cell(cell: [[usize; MAX_CELL]; MAX_CELL], j: usize, i: usize) -> bool {
    println!(" @{}@,  @{}@", j, i);
    return true;
}

fn view_cells(cells: [[usize; MAX_CELL]; MAX_CELL]) {
    for _j in 0..cells.len() {
        for _i in 0..cells.len() {
            print!("{}", convert_num_to_string(cells[_j][_i]));
        }
        println!("");
    }
}

fn main() {
    let init = 0;
    let cells: [[usize; MAX_CELL]; MAX_CELL] = [[init; MAX_CELL]; MAX_CELL];
    let mut new_cells: [[usize; MAX_CELL]; MAX_CELL] = [[init; MAX_CELL]; MAX_CELL];
    let mut rng = rand::thread_rng();

    let rule = LifeGameRule {
        birth: BIRTH_ARRAY,
        alive: ALIVE_ARRAY,
        death1: DEATH_ARRAY1,
        death2: DEATH_ARRAY2,
    };

    // UPDATE: NEW CELLS
    for n in 0..GENERATION {
        println!("GENERATION : _{}_", n + 1);
        for _j in 0..MAX_CELL {
            for _i in 0..MAX_CELL {
                let rand = rng.gen_range(0..2);
                // let is_alive = search_cell(cells, _j, _i);
                new_cells[_j][_i] = rand;
            }
            view_cells(new_cells);
            println!();
        }
        let mut _input_string = String::new();
        std::io::stdin().read_line(&mut _input_string).ok();
    }
}

