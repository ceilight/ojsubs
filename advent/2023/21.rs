use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let garden: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map(|l| {
            let l = l.unwrap();
            l.chars().collect::<Vec<_>>()
        })
        .collect();
    println!("Part 1: {}", part1(&garden));
    println!("Part 2: {}", part2(&garden));
}

type Point = (usize, usize);

// input observation (i'm not trusting the problem's author now):
// the garden is a 131x131 square grid, with starting point at the center
fn part1(garden: &[Vec<char>]) -> usize {
    let n = garden.len();
    let start: Point = (n / 2, n / 2);
    let mut to_visit = HashSet::from([start]);
    for _ in 0..64 {
        let mut new_to_visit = HashSet::new();
        for &(x, y) in to_visit.iter() {
            for (nx, ny) in [
                x.checked_sub(1).map(|x| (x, y)),
                (x < n - 1).then_some((x + 1, y)),
                y.checked_sub(1).map(|y| (x, y)),
                (y < n - 1).then_some((x, y + 1)),
            ]
            .iter()
            .flatten()
            {
                if garden[*nx][*ny] != '#' {
                    new_to_visit.insert((*nx, *ny));
                }
            }
        }
        to_visit.clear();
        to_visit.extend(new_to_visit);
    }
    to_visit.len()
}

fn fill_original_garden(garden: &[Vec<char>]) -> (usize, usize, usize) {
    let n = garden.len();
    let start: Point = (n / 2, n / 2);
    let mut to_visit: [HashSet<Point>; 2] = [HashSet::from([start]), HashSet::new()];
    for step in 0.. {
        let mut new_to_visit = HashSet::new();
        for &(x, y) in to_visit[step % 2].iter() {
            for (nx, ny) in [
                x.checked_sub(1).map(|x| (x, y)),
                (x < n - 1).then_some((x + 1, y)),
                y.checked_sub(1).map(|y| (x, y)),
                (y < n - 1).then_some((x, y + 1)),
            ]
            .iter()
            .flatten()
            {
                if garden[*nx][*ny] != '#' {
                    new_to_visit.insert((*nx, *ny));
                }
            }
        }
        if to_visit[(step + 1) % 2] == new_to_visit {
            return (step, to_visit[0].len(), to_visit[1].len());
        }
        to_visit[(step + 1) % 2].clear();
        to_visit[(step + 1) % 2].extend(new_to_visit);
    }
    unreachable!();
}

fn part2(garden: &[Vec<char>]) -> usize {
    let n = garden.len();
    assert_eq!(n, garden[0].len());
    let (steps, even_visits, odd_visits) = fill_original_garden(garden);
    assert_eq!(steps, n / 2 * 2);

    // The approach is to divide the resulting plot shape into tiles (with the original map
    // being the base tile) and calculate the area that is covered by the plot shape.
    // A tile can be either fully or partially covered by the plot shape. You can calculate
    // the area in partially covered tiles by filling the plot in the tile steps by steps
    // (similar to part 1's solution).
    // As for fully covered ones, pre-calculate the numbers of plots that are available
    // after odd/even number of steps. The numbers don't change after the plots cover the
    // entire original map.
    //
    // Start from the original map (marked *) and expand in all 4 cardinal directions
    // (U, D, L, R) and 4 ordinal directions (UL, UR, DL, DR). The timestamp (or step)
    // to begin filling adjacent tiles are the Manhattan distances between them and
    // the center of (*) (because one can only take one step north, south, east, or west)
    //
    // When dealing with diagonally-expanded tiles, remember to consider all tiles that are
    // not directly expanded vertically or horizontally as well
    //     0   1   2
    //   |---|---|---|
    // 0 |UL | U | UR|
    //   |-----------|
    // 1 | L | * | R |
    //   |-----------|---:---:--
    // 2 |DL | D |DR1|DR2:DR3:
    //           ----:---:---:--
    //           :DR2:DR3:DR4:
    //           :---:---:---:--
    //           :DR3:DR4:DR5:
    //           :---:---:---:--
    const TARGET_STEPS: usize = 26_501_365;

    // nearest point from the center of base tile
    #[rustfmt::skip]
    let fill_starts: [[Point; 3]; 3] = [
        //------L---------------------------R-------------
        [(n - 1, n - 1), (n - 1, n / 2), (n - 1, 0)], // U
        [(n / 2, n - 1), (n / 2, n / 2), (n / 2, 0)], // |
        [(0    , n - 1), (0    , n / 2), (0    , 0)], // D
    ];
    // number of steps to reach all points in a tile from the starting point
    // (manhattan distance from the starting point to the further point in the tile)
    let half = n / 2;
    let fill_depths: [[usize; 3]; 3] = [
        //---L--------------------R-------
        [half * 4, half * 3, half * 4], // U
        [half * 3, half * 2, half * 3], // |
        [half * 4, half * 3, half * 4], // D
    ];

    let fill_garden = |steps: usize, garden_pos: Point| -> usize {
        // case: the garden can be filled in under 26,501,365 steps
        let (i, j) = garden_pos;
        let steps_after_fill = steps + fill_depths[i][j];
        if steps_after_fill < TARGET_STEPS {
            if (TARGET_STEPS - steps_after_fill) % 2 == 0 {
                return even_visits;
            } else {
                return odd_visits;
            }
        }

        // case: the garden cannot be filled after 26,501,365 steps
        let mut to_visit = HashSet::new();
        to_visit.insert(fill_starts[i][j]);
        for _ in steps..TARGET_STEPS {
            let mut new_to_visit = HashSet::new();
            for &(x, y) in to_visit.iter() {
                for (nx, ny) in [
                    x.checked_sub(1).map(|x| (x, y)),
                    (x < n - 1).then_some((x + 1, y)),
                    y.checked_sub(1).map(|y| (x, y)),
                    (y < n - 1).then_some((x, y + 1)),
                ]
                .iter()
                .flatten()
                {
                    if garden[*nx][*ny] != '#' {
                        new_to_visit.insert((*nx, *ny));
                    }
                }
            }
            to_visit.clear();
            to_visit.extend(new_to_visit);
        }
        to_visit.len()
    };

    let mut result = if (TARGET_STEPS - steps) % 2 == 0 {
        even_visits
    } else {
        odd_visits
    };

    // horizontal and vertical expansion
    for step in (n / 2 + 1..=TARGET_STEPS).step_by(n) {
        result += fill_garden(step, (1, 0)); // left
        result += fill_garden(step, (1, 2)); // right
        result += fill_garden(step, (0, 1)); // up
        result += fill_garden(step, (2, 1)); // down
    }
    // diagonal expansion
    let mut i = 1;
    for step in (n + 1..=TARGET_STEPS).step_by(n) {
        result += fill_garden(step, (0, 0)) * i; // up-left
        result += fill_garden(step, (0, 2)) * i; // up-right
        result += fill_garden(step, (2, 0)) * i; // down-left
        result += fill_garden(step, (2, 2)) * i; // down-right
        i += 1;
    }

    result
}
