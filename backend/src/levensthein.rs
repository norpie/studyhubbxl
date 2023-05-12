/*Dit is de Search Algorithme */
/*
Bronnen:
-https://doc.rust-lang.org/stable/book/
-https://en.wikipedia.org/wiki/Levenshtein_distance
-https://en.wikipedia.org/wiki/Wagner%E2%80%93Fischer_algorithm
-https://www.dotnetperls.com/every-nth-element-rust
-https://youtu.be/gtoj6vOeb1A
-https://youtu.be/9VGM7wwf3JQ
-https://youtu.be/GcsAQTMYR1M
 */

use std::{
    cmp::{min, Ordering},
    result,
};

use actix_web::web::Query;

fn levensthein_distance(s1: &str, s2: &str) -> usize {
    let x = s1.len();
    let y = s2.len();

    let mut a: Vec<Vec<usize>> = vec![vec![0; x + 1]; y + 1];

    for i in 0..x {
        a[i][0] = i;
    }
    for j in 0..y {
        a[0][j] = j;
    }

    for i in 1..=x {
        for j in 1..y {
            let substitution_cost = if s1.chars().nth(i - 1) == s2.chars().nth(j - 1) {
                0
            } else {
                1
            };

            a[i][j] = min(
                a[i - 1][j] + 1,
                min(a[i][j - 1] + 1, a[i - 1][j - 1] + substitution_cost),
            );
        }
    }

    a[x][y]
}

// derde keer de goeie keer?
//attemp 3 :
fn search_closest_strings<'a>(
    query: &str,
    candidates: Vec<&'a str>,
    max_distance: usize,
) -> Vec<&'a str> {
    let mut result: Vec<(&str, usize)> = Vec::new();

    for candidate in candidates.iter() {
        let distance = levensthein_distance(query, candidate);

        if distance <= max_distance {
            result.push((candidate, distance));
        }
    }

    result.sort_unstable_by_key(|(_, distance)| *distance);

    result.iter().map(|(candidate, _)| *candidate).collect()
}
/*
[1:36 PM] KUOSMANEN Konsta (s)
je wilt dus een functie die een lijst van opties krijgt als strings, en een string, de search term,
​[1:36 PM] KUOSMANEN Konsta (s)
    je loopt door elke optie en berekent de levenshtein
​[1:36 PM] KUOSMANEN Konsta (s)
    je zet elke optie in een hashmap samen met de levenshtein
​[1:37 PM] KUOSMANEN Konsta (s)
    dan sorteer je de hashmap gebaseerd op de levenshtein
​[1:37 PM] KUOSMANEN Konsta (s)
    dan return je de hashmap
​[1:37 PM] KUOSMANEN Konsta (s)
    of een vec

 */

//attempt 1 :
/*fn search_string(query: String, candidates: String, max_distance: usize) String -> Vec<String, usize> {
    let mut results: Vec<(String, usize)> = Vec::new();

    for candidate in candidates {
        let distance = levensthein_distance(&query, candidate);

        if distance <= max_distance {
            results.push((candidate, distance));
        }
    }

    results.sort_unstable_by(|a, b| a.1.cmp(&b.1));
    results.iter().map(|(candidate, _)| *candidate).collect();
}*/

//attempt 2:
/*fn search_closest_strings(query: String, candidates: &[&str], max_distance: usize) -> Vec<String> {
    let mut result: Vec<(String, usize)> = Vec::new();

    for candidate in candidates {
        let disatnce = levensthein_distance(&query, &candidate);

        if distance <= max_distance {
            result.push((candidate, &distance));
        }
    }
    result.sort_unstable_by(|a, b| a.1.cmp(&b.1));
    result.iter().map(|(candidate, _)| *candidate).collect()
}*/
