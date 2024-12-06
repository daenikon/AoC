pub fn run(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let mut counter = 0;
    let rows = grid.len();
    let cols = grid[0].len();

    let directions: [[isize; 2]; 8] = [
        [0, 1],
        [1, 0],
        [0, -1],
        [-1, 0],
        [1, 1],
        [1, -1],
        [-1, -1],
        [-1, 1],
    ];

    for row in 0..rows {
        for col in 0..cols {
            counter += count_xmas(&grid, (row, col), &directions);
        }
    }

    counter
}

fn count_xmas(
    grid: &Vec<Vec<char>>,
    (r, c): (usize, usize),
    directions: &[[isize; 2]; 8],
) -> u32 {
    if grid[r][c] != 'X' {
        return 0;
    }

    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    let goal = ['M', 'A', 'S'];
    let mut total_found = 0;

    for dir in directions {
        let mut is_match = true;
        for (i, &ch) in goal.iter().enumerate() {
            let nr = r as isize + (i as isize + 1) * dir[0];
            let nc = c as isize + (i as isize + 1) * dir[1];

            if nr < 0 || nr >= rows || nc < 0 || nc >= cols {
                is_match = false;
                break;
            }

            if grid[nr as usize][nc as usize] != ch {
                is_match = false;
                break;
            }
        }

        if is_match {
            total_found += 1;
        }
    }

    total_found
}

