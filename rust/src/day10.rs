use crate::utils;

pub fn farthest_steps() -> u64 {
    let input = utils::read_input_file("./inputs/10_test1.txt");

    let (mut mat, (s_x, s_y)) = parse_into_matrix(input);

    // part 1
    let steps = bfs(&mut mat, (s_x as i32, s_y as i32));
    // return steps;

    // part 2

    count_enclosed_tiles(&mut mat)
}

fn parse_into_matrix(input_str: String) -> (Vec<Vec<char>>, (usize, usize)) {
    let mut mat = Vec::new();
    let (mut s_x, mut s_y) = (0usize, 0usize);

    input_str.lines().enumerate().for_each(|(i, line)| {
        let line_vec: Vec<char> = line.chars().collect();

        if let Some((j, _)) = line_vec.iter().enumerate().find(|(_, c)| **c == 'S') {
            s_x = i;
            s_y = j;
        }
        mat.push(line_vec);
    });

    (mat, (s_x, s_y))
}

fn bfs(mat: &mut Vec<Vec<char>>, start: (i32, i32)) -> u64 {
    let (s_x, s_y) = start;

    let mut queue1 = (-1, -1);
    let mut queue2 = (-1, -1);

    // step1: search around S
    mat[s_x as usize][s_y as usize] = '#';

    let left = if s_y > 0 {
        Some(mat[s_x as usize][s_y as usize - 1])
    } else {
        None
    };

    let right = if (s_y as usize) < mat[0].len() - 1 {
        Some(mat[s_x as usize][s_y as usize + 1])
    } else {
        None
    };

    let top = if s_x > 0 {
        Some(mat[s_x as usize - 1][s_y as usize])
    } else {
        None
    };

    let bottom = if (s_x as usize) < mat.len() - 1 {
        Some(mat[s_x as usize + 1][s_y as usize])
    } else {
        None
    };

    let mut candis = Vec::new();

    if let Some(left) = left {
        if left == '-' || left == 'L' || left == 'F' {
            candis.push((s_x, s_y - 1));
        }
    }

    if let Some(right) = right {
        if right == '-' || right == 'J' || right == '7' {
            candis.push((s_x, s_y + 1));
        }
    }

    if let Some(top) = top {
        if top == '|' || top == 'F' || top == '7' {
            candis.push((s_x - 1, s_y));
        }
    }

    if let Some(bottom) = bottom {
        if bottom == '|' || bottom == 'J' || bottom == 'L' {
            candis.push((s_x + 1, s_y));
        }
    }

    assert!(candis.len() == 2);

    queue1 = candis.pop().unwrap();
    queue2 = candis.pop().unwrap();

    // step2: normal bfs till the two meet
    let mut steps = 1;

    while mat[queue1.0 as usize][queue1.1 as usize] != '#' {
        // queue1
        queue1 = find_next_step(mat, queue1);

        // queue2
        queue2 = find_next_step(mat, queue2);

        steps += 1;

        if queue1 == queue2 {
            find_next_step(mat, queue1);
            break;
        }
    }

    steps
}

fn find_next_step(mat: &mut Vec<Vec<char>>, curr: (i32, i32)) -> (i32, i32) {
    let (x, y) = curr;

    assert!(x >= 0 && x < mat.len() as i32 && y >= 0 && y < mat[0].len() as i32);

    let curr_char = mat[x as usize][y as usize];
    mat[x as usize][y as usize] = '#';

    match curr_char {
        '|' => {
            let up = mat[x as usize - 1][y as usize];

            if up == '#' {
                (x + 1, y)
            } else {
                (x - 1, y)
            }
        }
        '-' => {
            let left = mat[x as usize][y as usize - 1];

            if left == '#' {
                (x, y + 1)
            } else {
                (x, y - 1)
            }
        }
        'L' => {
            let up = mat[x as usize - 1][y as usize];

            if up == '#' {
                (x, y + 1)
            } else {
                (x - 1, y)
            }
        }
        'J' => {
            let up = mat[x as usize - 1][y as usize];

            if up == '#' {
                (x, y - 1)
            } else {
                (x - 1, y)
            }
        }
        '7' => {
            let left = mat[x as usize][y as usize - 1];

            if left == '#' {
                (x + 1, y)
            } else {
                (x, y - 1)
            }
        }
        'F' => {
            let right = mat[x as usize][y as usize + 1];

            if right == '#' {
                (x + 1, y)
            } else {
                (x, y + 1)
            }
        }
        _ => {
            panic!("unexpected char");
        }
    }
}

fn count_enclosed_tiles(mat: &mut Vec<Vec<char>>) -> u64 {
    let mut count = 0;

    for x in 0..mat.len() {
        let mut is_odd = 0;

        for y in 0..mat[0].len() {
            let cell = mat[x][y];

            if cell == '#' {
                // case 1: #(path of the loop)
                is_odd ^= 1;
            }

            // case 2: cell == '.'
            if cell == '.' {
                if is_odd == 1 {
                    dbg!(&x, &y);
                    count += 1;
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::parse_into_matrix;

    #[test]
    fn test_parse_into_matrx() {
        let input = String::from("...\n.S.\nABC");
        let (vec, (s_x, s_y)) = parse_into_matrix(input);

        assert_eq!((s_x, s_y), (1, 1));
        assert_eq!(
            vec,
            vec![
                vec!['.', '.', '.'],
                vec!['.', 'S', '.'],
                vec!['A', 'B', 'C']
            ]
        );
    }
}
