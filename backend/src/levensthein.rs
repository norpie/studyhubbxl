/*Dit is de Search Algorithme */

use std::cmp::{min, Ordering};

fn levensthein_distance(s1: &str, s2: &str) -> usize {
    let x = s1.len();
    let y = s2.len();

    let mut vector_a: Vec<Vec<usize>> = vec![vec![0; x + 1]; y + 1];

    for i in 0..x {
        vector_a[i][0] = i;
    }
    for j in 0..y {
        vector_a[0][j] = j;
    }

    for i in 1..=x {
        for j in 1..y {
            let substitution_cost = if s1.chars().nth(i - 1) == s2.chars().nth(j - 1) {
                0
            } else {
                1
            };

            vector_a[i][j] = min(
                vector_a[i - 1][j] + 1,
                min(
                    vector_a[i][j - 1] + 1,
                    vector_a[i - 1][j - 1] + substitution_cost,
                ),
            );
        }
    }

    vector_a[x][y]
}
