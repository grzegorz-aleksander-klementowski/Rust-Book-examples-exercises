// Given a list of integers, use a vector and return the median
// (when sorted, the value in the middle position) and mode
// (the value that occurs most often; a hash map will be helpful here) of the list.

use std::{collections::HashMap, hash::Hash};

#[allow(dead_code)]
fn cal_mode<T: Eq + Hash + Clone>(zbiór: Vec<T>) -> Option<T> {
    let mut rys: HashMap<T, usize> = HashMap::new();

    for wskaźnik in zbiór {
        *rys.entry(wskaźnik).or_insert(0) += 1;
    }
    let najwyższa_wartość = *rys.values().clone().max()?;

    let wynik = rys.iter().clone().find(|(_, v)| **v == najwyższa_wartość)?;

    Some(wynik.0.clone())
}

#[allow(dead_code)]
fn cal_median_from_vec_integers(mut set: Vec<u32>) -> f32 {
    // sorting the numbers in a set.
    set.sort();
    // the length of a vec
    let n = set.len();

    // if is even or else odd
    if n % 2 == 0 {
        let wskaźnik_1 = (n / 2) - 1; //-1 to represent the real set indexing
        let wskaźnik_2 = ((n / 2) + 1) - 1; // -1 to represent the real set indexing

        let x1 = *set.get(wskaźnik_1).unwrap() as f32;
        let x2 = *set.get(wskaźnik_2).unwrap() as f32;

        (x1 + x2) / 2.0
    } else {
        let mut wskaźnik: usize = 0;
        wskaźnik += (n / 2) + 1;
        wskaźnik -= 1; // -1 to represent the real set indexing
        wskaźnik as usize;
        *set.get(wskaźnik).unwrap() as f32
    }
}

fn main() {
    println!("The programming is writen in tests. Write `cargo test`.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_median_from_even_vector() {
        let vec_int_even: Vec<u32> = vec![4, 10, 5, 1, 3, 6, 9, 8, 7, 2];
        let median = cal_median_from_vec_integers(vec_int_even);
        assert_eq!(5.5, median)
    }

    #[test]
    fn test_calculate_median_from_odd_vector() {
        let vec_int_odd: Vec<u32> = vec![4, 0, 10, 5, 1, 3, 6, 9, 8, 7, 2];
        let median = cal_median_from_vec_integers(vec_int_odd);
        assert_eq!(5.0, median)
    }

    #[test]
    fn test_cal_mode() {
        let kierunkowy = vec![
            4, 7, 10, 5, 7, 1, 3, 7, 9, 8, 7, 2, 7, 4, 1, 3, 4, 1, 4, 7, 2, 2, 1,
        ];
        let najczęśtrzy = cal_mode(kierunkowy);
        assert_eq!(Some(7), najczęśtrzy)
    }
}
