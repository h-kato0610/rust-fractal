use rand::Rng;

const GENERATION: usize = 2;

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
        ALIVE_NUM => result = ALIVE_CELL,
        DEAD_NUM => result = DEAD_CELL,
        _ => result = DEAD_CELL,
    }

    return result.to_string();
}

fn search_cell(cells: &[[usize; MAX_CELL]; MAX_CELL], _j: usize, _i: usize, rule: &LifeGameRule) -> usize { // [[usize; MAX_CELL]; MAX_CELL] {
    let current_cell_is_alive_or_death = cells[_j][_i];
    let mut top: usize;
    let mut left: usize;
    let mut left_top: usize;
    let mut left_bottom: usize;
    let mut right: usize;
    let mut right_top: usize;
    let mut right_bottom: usize;
    let mut bottom: usize;

    let init: usize = 0;

    let mut new_cells: [[usize; MAX_CELL]; MAX_CELL] = [[init; MAX_CELL]; MAX_CELL];
    if _i == 0 {
        left = DEAD_NUM;
    } else {
        left = cells[_j][_i - 1];
    }

    if _i == MAX_CELL {
        right = DEAD_NUM;
    } else {
        right = cells[_j][_i];
    }

    if _j == 0 {
        top = DEAD_NUM;
    } else {
        top = cells[_j][_i];
    }

    if _j == MAX_CELL {
        bottom = DEAD_NUM;
    } else {
        bottom = cells[_j][_i];
    }

    if _j == 0 || _i == 0 {
        left_top = DEAD_NUM;
    } else {
        left_top = cells[_j][_i];
    }

    if _j == 0 || _i == MAX_CELL {
        left_bottom = DEAD_NUM;
    } else {
        left_bottom = cells[_j][_i];
    }

    if _j == MAX_CELL || _i == MAX_CELL {
        right_top = DEAD_NUM;
    } else {
        right_top = cells[_j][_i];
    }

    if _j == MAX_CELL || _i == MAX_CELL {
        right_bottom = DEAD_NUM;
    } else {
        right_bottom = cells[_j][_i];
    }

    let adjacent_cells = top + left + left_top + left_bottom + right + right_top + right_bottom + bottom;

    println!("current: {} , adj : {}", current_cell_is_alive_or_death, adjacent_cells);

    let next_cell = 
        if current_cell_is_alive_or_death == DEAD_NUM {
            if adjacent_cells == 3 {
                return ALIVE_NUM;
            }
        }
        else if current_cell_is_alive_or_death == ALIVE_NUM {
            if adjacent_cells == 3 || adjacent_cells == 2 {
                return ALIVE_NUM;
            }
            else if ALIVE_NUM <= 1 {
                return DEAD_NUM;
            }
            else {
                return DEAD_NUM;
            }
        }
        else {
            return DEAD_NUM;
        };
    return DEAD_NUM;
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
    // let mut cells: [[usize; MAX_CELL]; MAX_CELL] = [[init; MAX_CELL]; MAX_CELL];
    let mut cells: [[usize; MAX_CELL]; MAX_CELL] = [[init; MAX_CELL]; MAX_CELL];
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
        println!("GENERATION : {}", n + 1);
        for _j in 0..MAX_CELL {
            for _i in 0..MAX_CELL {
                if n == 0 {
                    let rand = rng.gen_range(0..2);
                    cells[_j][_i] = rand;
                } else {
                    new_cells[_j][_i] = search_cell(&cells, _j, _i, &rule);
                }
            }
        }

        view_cells(cells);
        println!();

        cells = new_cells.clone();

        let mut _input_string = String::new();
        std::io::stdin().read_line(&mut _input_string).ok();
    }
}

