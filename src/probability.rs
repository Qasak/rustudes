use std::collections::HashSet;
use std::hash::Hash;
use itertools::Itertools;
use std::str;
use rand::seq::index;
type F = fraction::Fraction;


/// "The probability of an event, given a sample space of equiprobable outcomes."
pub fn P <T: Eq + Hash>(event: Vec<T>, space: Vec<T>) -> F {
    let event_set: HashSet<T> = event.into_iter().collect();
    let space_set: HashSet<T> = space.into_iter().collect();
    let l1 = event_set.intersection(&space_set).collect::<HashSet<_>>().len();
    let l2 = space_set.len();
    F::from(l1) / F::from(l2)
}



/// "The set of ways of concatenating one item from collection A with one from B."
pub fn cross(A: &str, B: &str) -> Vec<String> {
    let mut list = Vec::new();
    for &a in A.as_bytes() {
        for &b in B.as_bytes() {
            let s = match String::from_utf8([a, b].to_vec()) {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };
            list.push(s);
        }
    }
    list
}

/// "All combinations of n items; each combo as a concatenated str."
pub fn combos (items: Vec<String>, n: usize) -> Vec<String> {
    let its = items
        .into_iter()
        .combinations(n)
        .into_iter()
        .collect::<Vec<_>>();
    let mut ret = vec![];
    for it in its {
        ret.push(it.join(" "));
    }
    ret
}



#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use rand::seq::IteratorRandom;
    use rand::thread_rng;
    use super::*;
    #[test]
    fn even() {
        let D = vec![1, 2, 3, 4, 5, 6];
        let even = vec![   2,    4,    6];
        println!("{:?}", P(even, D));
    }

    #[test]
    fn even_2() {
        let D =    vec![1, 2, 3, 4, 5, 6];
        let even = vec![2, 4, 6, 8, 10, 12];
        println!("{:?}", P(even, D));
    }

    fn get_urn() -> Vec<String>{
        let W = cross("W", "12345678");
        let B = cross("B", "123456");
        let R = cross("R", "123456789");
        let urn = vec![W, B, R].into_iter().flatten().collect::<Vec<_>>();
        println!("{:?}", urn);
        urn
    }

    #[test]
    fn get_combos() {
        let urn = get_urn();
        let U6 = combos(urn, 6);
        let mut rng = thread_rng();
        let sample = U6.iter().choose_multiple(&mut rng, 10);
        println!("{:?}", U6.len());
        println!("{:?}", sample);
    }

    #[test]
    fn q_1() {
        let urn = get_urn();
        let U6 = combos(urn, 6);
        let red6 = U6.clone()
            .into_iter()
            .filter(
                |s| s.bytes().filter(|&b| b == b'R').count() == 6
            )
            .collect::<Vec<String>>();
        println!("{:?}", red6);
        println!("{:?}", P(red6, U6));
    }

}

