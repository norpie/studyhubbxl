/*
Bronnen:
    - https://doc.rust-lang.org/stable/book/
    - https://en.wikipedia.org/wiki/Levenshtein_distance
    - https://en.wikipedia.org/wiki/Wagner%E2%80%93Fischer_algorithm
    - https://www.dotnetperls.com/every-nth-element-rust
    - https://youtu.be/gtoj6vOeb1A
    - https://youtu.be/9VGM7wwf3JQ
    - https://youtu.be/GcsAQTMYR1M
 */

use std::{
    cmp::{min, Ordering},
    result,
};

use actix_web::web::Query;

fn levensthein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.len();
    let len2 = s2.len();

    let mut distances: Vec<Vec<usize>> = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 0..=len1 {
        distances[i][0] = i;
    }
    for j in 0..=len2 {
        distances[0][j] = j;
    }

    for j in 1..=len2 {
        for i in 1..=len1 {
            let substitution_cost = if s1.chars().nth(i - 1) == s2.chars().nth(j - 1) {
                0
            } else {
                1
            };

            distances[i][j] = min(
                distances[i - 1][j] + 1,
                min(distances[i][j - 1] + 1, distances[i - 1][j - 1] + substitution_cost),
            );
        }
    }

    distances[len1][len2]
}

fn search_closest_strings<'a>(
    query: &str,
    candidates: Vec<&'a str>,
    max_distance: usize,
) -> Vec<&'a str> {
    let mut result: Vec<(&str, usize)> = Vec::new();

    for candidate in candidates {
        let distance = levensthein_distance(query, candidate);

        if distance <= max_distance {
            result.push((candidate, distance));
        }
    }

    result.sort_unstable_by_key(|(_, distance)| *distance);

    result.iter().map(|(candidate, _)| *candidate).collect()
}

#[cfg(test)]
mod tests {
    use crate::levensthein::{levensthein_distance, search_closest_strings};

    #[test]
    fn test_levenshtein1() {
        let result = levensthein_distance("kitten", "sitten");
        assert_eq!(result, 1);
    }
    #[test]
    fn test_levenshtein2() {
        let result = levensthein_distance("kitten", "sitting");
        assert_eq!(result, 3);
    }
    #[test]
    fn test_search_closest_strings() {
        let result = search_closest_strings("kitten", vec!["kitten", "sitting", "sitten"], 99);
        assert_eq!(result, vec!["kitten", "sitten", "sitting"]);
    }
    #[test]
    fn test_search_closest_strings2() {
        let result = search_closest_strings("kitten", vec!["kitten", "sitting", "sitten"], 2);
        assert_eq!(result, vec!["kitten", "sitten"]);
    }
}
