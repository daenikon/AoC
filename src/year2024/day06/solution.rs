use std::collections::HashMap;

pub fn run(input: &str) -> u32 {
    let mut grid: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    // 1 becauce of the last outer-layer cell, which is inconvenient to count in the loop
    let mut pos_cnt = 1;
    let mut guard_dir: char = '<';
    let rows = grid.len();
    let cols = grid[0].len();
    let arrows: HashMap<char, [i32; 2]> =
        HashMap::from([('^', [0, 1]), ('>', [1, 0]), ('v', [0, -1]), ('<', [-1, 0])]);
    let mut position: [usize; 2] = [0, 0];

    // get initial position of the guard
    for row in 0..rows {
        for col in 0..cols {
            for &arrow in arrows.keys() {
                if grid[row][col] == arrow {
                    position = [row, col];
                    //guard_dir = arrow;
                    //println!("{:?}", position);
                }
            }
        }
    }

    while position[0] != 0 && position[0] != rows - 1 && position[1] != 0 && position[1] != cols - 1
    {
        let cur_char = grid[position[0]][position[1]];

        // increment count only if guard wasn't on that cell before
        // cell that have been walked before are marked with *
        if cur_char != '*' {
            grid[position[0]][position[1]] = '*';
            pos_cnt += 1;
        } 

        if let Some(&[dx, dy]) = arrows.get(&guard_dir) {
            let next_position = [(position[0] as i32 + dx) as usize, (position[1] as i32 + dy) as usize];
            if grid[next_position[0]][next_position[1]] != '#' {
                position[0] = (position[0] as i32 + dx) as usize;
                position[1] = (position[1] as i32 + dy) as usize;
            } else {
                guard_dir = rotate(guard_dir);
            }
        } else {
            println!("Invalid guard direction: {}", guard_dir);
            break;
        }
        //println!("{} .. {}", rows - 1, cols - 1);
        //println!("{} and {}", cur_char, grid[position[0]][position[1]]);
    }

    /*
    let grid_str: String = grid
        .iter()
        .flat_map(|row| row.iter())
        .collect();  
    */
    pos_cnt
}

fn rotate(sym: char) -> char {
    if sym == '^' {
        return '>';
    } else if sym == '>' {
        return 'v';
    } else if sym == 'v' {
        return '<';
    } else {
        '^'
    }
}
