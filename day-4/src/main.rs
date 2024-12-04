fn main() {
    let input = include_str!("../input.txt");
    let result = part_1(input);
    let result_2 = part_2(input);
    println!("Total XMAS: {}", result);
    println!("Total X-MAS: {}", result_2);
}

fn part_1(input: &str) -> i32 {
    let mtx: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut total_xmas = 0;

    let samx = "SAMX"; // Reversed just because that's how I wrote it

    for row in 0..mtx.len() {
        for col in 0..mtx[row].len() {
            if mtx[row][col] != 'X' {
                continue;
            }

            println!("Found X at {},{}", row, col);

            // Note: I realised I made all these x & y in reverse
            // and i swapped it in part 2 but this shit was bad enough
            if col >= 3 {
                let mx = col - 3;
                if mtx[row].get(mx).is_some() {
                    let w: String = vec![
                        mtx[row][mx],
                        mtx[row][mx + 1],
                        mtx[row][mx + 2],
                        mtx[row][mx + 3],
                    ]
                    .iter()
                    .collect();
                    if w == samx {
                        total_xmas += 1;
                    }
                }
            }

            if col + 3 < mtx[row].len() {
                let px = col + 3;
                if mtx[row].get(px).is_some() {
                    let w: String = vec![
                        mtx[row][px],
                        mtx[row][px - 1],
                        mtx[row][px - 2],
                        mtx[row][px - 3],
                    ]
                    .iter()
                    .collect();
                    if w == samx {
                        total_xmas += 1;
                    }
                }
            }

            if row >= 3 {
                let my = row - 3;
                if mtx.get(my).and_then(|v| v.get(col)).is_some() {
                    let w: String = vec![
                        mtx[my][col],
                        mtx[my + 1][col],
                        mtx[my + 2][col],
                        mtx[my + 3][col],
                    ]
                    .iter()
                    .collect();
                    if w == samx {
                        total_xmas += 1;
                    }
                }
            }

            if row + 3 < mtx.len() {
                let py = row + 3;
                dbg!(row, py);
                let w: String = vec![
                    mtx[py][col],
                    mtx[py - 1][col],
                    mtx[py - 2][col],
                    mtx[py - 3][col],
                ]
                .iter()
                .collect();
                if w == samx {
                    total_xmas += 1;
                }
            }

            if row >= 3 && col >= 3 {
                let mdx = row - 3;
                let mdy = col - 3;
                if mtx.get(mdx).and_then(|v| v.get(mdy)).is_some() {
                    let w: String = vec![
                        mtx[mdx][mdy],
                        mtx[mdx + 1][mdy + 1],
                        mtx[mdx + 2][mdy + 2],
                        mtx[mdx + 3][mdy + 3],
                    ]
                    .iter()
                    .collect();
                    if w == samx {
                        total_xmas += 1;
                    }
                }
            }

            if row + 3 < mtx.len() && col + 3 < mtx[row].len() {
                let pdx = row + 3;
                let pdy = col + 3;
                if mtx.get(pdx).and_then(|v| v.get(pdy)).is_some() {
                    let w: String = vec![
                        mtx[pdx][pdy],
                        mtx[pdx - 1][pdy - 1],
                        mtx[pdx - 2][pdy - 2],
                        mtx[pdx - 3][pdy - 3],
                    ]
                    .iter()
                    .collect();
                    if w == samx {
                        total_xmas += 1;
                    }
                }
            }

            if row >= 3 && col + 3 < mtx[row].len() {
                let pdx = row - 3;
                let pdy = col + 3;
                if mtx.get(pdx).and_then(|v| v.get(pdy)).is_some() {
                    let w: String = vec![
                        mtx[pdx][pdy],
                        mtx[pdx + 1][pdy - 1],
                        mtx[pdx + 2][pdy - 2],
                        mtx[pdx + 3][pdy - 3],
                    ]
                    .iter()
                    .collect();
                    if w == samx {
                        total_xmas += 1;
                    }
                }
            }

            if row + 3 < mtx.len() && col >= 3 {
                let pdx = row + 3;
                let pdy = col - 3;
                if mtx.get(pdx).and_then(|v| v.get(pdy)).is_some() {
                    let w: String = vec![
                        mtx[pdx][pdy],
                        mtx[pdx - 1][pdy + 1],
                        mtx[pdx - 2][pdy + 2],
                        mtx[pdx - 3][pdy + 3],
                    ]
                    .iter()
                    .collect();
                    if w == samx {
                        total_xmas += 1;
                    }
                }
            }
        }
    }
    return total_xmas;
}

fn part_2(input: &str) -> i32 {
    let mtx: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut total_xmas = 0;

    for row in 0..mtx.len() {
        for col in 0..mtx[row].len() {
            if mtx[row][col] != 'A' {
                continue;
            }

            let mut left_cross: Option<String> = None;
            let mut right_cross: Option<String> = None;

            println!("Found A at {},{}", row+1, col+1);

            // An a needs at 1 column and 1 row in either direction
            if row < 1 || row + 2 > mtx.len() {
                continue;
            }
            if col < 1 || col + 2 > mtx[row].len() {
                continue;
            }

            let pdy = row - 1;
            let pdx = col + 1;
            if mtx.get(pdy).and_then(|v| v.get(pdx)).is_some() {
                right_cross = Some(
                    vec![mtx[pdy][pdx], mtx[pdy + 1][pdx - 1], mtx[pdy + 2][pdx - 2]]
                        .iter()
                        .collect::<String>(),
                );

                println!("RIGHT");
                for ch in right_cross.clone().unwrap().chars() {
                    println!("{}", ch);
                }
            }

            let mdy = row - 1;
            let mdx = col - 1;
            if mtx.get(mdy).and_then(|v| v.get(mdx)).is_some() {
                left_cross = Some(
                    vec![mtx[mdy][mdx], mtx[mdy + 1][mdx + 1], mtx[mdy + 2][mdx + 2]]
                        .iter()
                        .collect::<String>(),
                );
                println!("LEFT");
                for ch in left_cross.clone().unwrap().chars() {
                    println!("{}", ch);
                }
            }
            if let Some(lx) = left_cross {
                if let Some(rx) = right_cross {
                    if (lx == "MAS" || lx == "SAM") && ( rx == "SAM" || rx == "MAS") {
                        total_xmas += 1;
                        println!("Found XMAS - starting {},{} and ending {},{}", mdy+1,mdx+1,pdy+1,pdx+1);
                    }
                }
            }
        }
    }
    return total_xmas;
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT), 18)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT), 9)
    }
}
