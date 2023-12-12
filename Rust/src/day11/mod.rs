use itertools::Itertools;

#[derive(Debug)]
struct Galaxy {
    x: isize,
    y: isize,
}

impl Galaxy {
    fn manhattan_distance(&self, other: &Galaxy) -> usize {
        return ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize;
    }

    fn mins(&self, other: &Galaxy) -> (isize, isize) {
        return (self.x.min(other.x), self.y.min(other.y));
    }

    fn maxes(&self, other: &Galaxy) -> (isize, isize) {
        return (self.x.max(other.x), self.y.max(other.y));
    }
}

pub fn solve1(input: &String) {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    let empty_rows = grid
        .iter()
        .enumerate()
        .filter(|(_, r)| r.iter().all(|c| *c == '.'))
        .map(|(y, _)| y as usize)
        .collect::<Vec<_>>();

    let empty_cols = (0..grid[0].len())
        .filter(|x| grid.iter().all(|r| r[*x] == '.'))
        .map(|x| x as usize)
        .collect::<Vec<_>>();

    let new_rows: usize = rows + empty_rows.len();
    let new_cols: usize = cols + empty_cols.len();

    let mut new_grid: Vec<Vec<char>> = Vec::new();

    for i in 0..rows {
        let mut new_line: Vec<char> = Vec::new();
        for j in 0..cols {
            if empty_cols.contains(&j) {
                new_line.push('.');
            }
            new_line.push(grid[i][j]);
        }

        new_grid.push(new_line.clone());

        if empty_rows.contains(&i) {
            new_grid.push(new_line.clone());
        }
    }

    for i in 0..new_rows {
        for j in 0..new_cols {
            print!("{}", new_grid[i][j]);
        }
        println!();
    }

    let galaxies: Vec<Galaxy> = grid
        .iter()
        .enumerate()
        .flat_map(|(y, r)| {
            r.iter()
                .enumerate()
                .filter(|(_, c)| **c == '#')
                .map(move |(x, _)| Galaxy {
                    x: x as isize,
                    y: y as isize,
                })
        })
        .collect::<Vec<_>>();

    let galaxy_pairs: Vec<(&Galaxy, &Galaxy)> = galaxies.iter().tuple_combinations().collect_vec();

    println!(
        "{:?}",
        galaxy_pairs
            .iter()
            .map(|(src, dst)| src.manhattan_distance(dst))
            .sum::<usize>()
    );
}

pub fn solve2(input: &String) {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let empty_rows = grid
        .iter()
        .enumerate()
        .filter(|(_, r)| r.iter().all(|c| *c == '.'))
        .map(|(y, _)| y as isize)
        .collect::<Vec<_>>();

    let empty_cols = (0..grid[0].len())
        .filter(|x| grid.iter().all(|r| r[*x] == '.'))
        .map(|x| x as isize)
        .collect::<Vec<_>>();

    let galaxies: Vec<Galaxy> = grid
        .iter()
        .enumerate()
        .flat_map(|(y, r)| {
            r.iter()
                .enumerate()
                .filter(|(_, c)| **c == '#')
                .map(move |(x, _)| Galaxy {
                    x: x as isize,
                    y: y as isize,
                })
        })
        .collect::<Vec<_>>();

    let scale = 999_999;
    let galaxy_pairs: Vec<(&Galaxy, &Galaxy)> = galaxies.iter().tuple_combinations().collect_vec();
    let mut result = 0;

    for (g1, g2) in galaxy_pairs.iter() {
        let dist = g1.manhattan_distance(g2);

        let (min_x, min_y) = g1.mins(g2);
        let (max_x, max_y) = g1.maxes(g2);

        // counts how many rows and cols there are in between the galaxy pair and multiplies them by the expansion scale

        let rows = empty_rows
            .iter()
            .filter(|r| min_y < **r && max_y > **r)
            .count()
            * scale;

        let cols = empty_cols
            .iter()
            .filter(|c| min_x < **c && max_x > **c)
            .count()
            * scale;

        result += dist + rows + cols;
    }

    println!("Result: {}", result);
}
