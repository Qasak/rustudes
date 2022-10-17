use std::collections::HashSet;
use std::hash::Hash;
use itertools::Itertools;
use std::str;
use rand::seq::index;
type F = fraction::Fraction;


///     """The probability of an event, given a sample space of equiprobable outcomes.
//     event can be either a set of outcomes, or a predicate (true for outcomes in the event)."""
pub fn P <T: Eq + Hash>(event: &Vec<T>, space: &Vec<T>) -> F {
    let event_set: HashSet<&T> = event.into_iter().collect();
    let space_set: HashSet<&T> = space.into_iter().collect();
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

pub fn choose(n: u128, c: u128) -> u128 {
    factorial(n) / (factorial(n - c) * factorial(c))
}

pub fn factorial(n: u128) -> u128 {
    let mut n = n;
    let mut ret = 1;
    while n > 0 {
        ret *= n;
        n -= 1;
    }
    ret
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
    use std::io::Read;
    use std::ops::Deref;
    use fraction::Fraction;
    use rand::seq::IteratorRandom;
    use rand::thread_rng;
    use super::*;
    #[test]
    fn even() {
        let D = vec![1, 2, 3, 4, 5, 6];
        let even = vec![2,    4,    6];
        println!("{:?}", P(&even, &D));
    }

    #[test]
    fn even_2() {
        let D =    vec![1, 2, 3, 4, 5, 6];
        let even = vec![2, 4, 6, 8, 10, 12];
        println!("{:?}", P(&even, &D));
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
    fn c() {
        println!("{}", choose(23, 6));
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
        println!("{:?}", P(&red6, &U6));
        assert_eq!(P(&red6, &U6), F::from(choose(9, 6)) / F::from(U6.len()));
    }

    #[test]
    fn q_2() {
        let urn = get_urn();
        let U6 = combos(urn, 6);
        let b3w2r1 = U6.clone()
            .into_iter()
            .filter(
                |s| s.bytes()
                    .filter(|&b| b == b'B').count() == 3
            )
            .filter(
                |s| s.bytes()
                    .filter(|&b| b == b'W').count() == 2
            ).filter(
                |s| s.bytes()
                    .filter(|&b| b == b'R').count() == 1
            )
            .collect::<Vec<String>>();
        println!("{:?}", b3w2r1);
        println!("{:?}", P(&b3w2r1, &U6));
        assert_eq!(P(&b3w2r1, &U6),
                   F::from(choose(6, 3) * choose(8, 2) * choose(9, 1) ) / F::from(U6.len()));
    }

    #[test]
    fn q_3() {
        let urn = get_urn();
        let U6 = combos(urn, 6);
        let w4 = U6.clone()
            .into_iter()
            .filter(
                |s| s.bytes()
                    .filter(|&b| b == b'W').count() == 4
            ).collect::<Vec<String>>();
        println!("{:?}", w4);
        println!("{:?}", P(&w4, &U6));
        assert_eq!(P(&w4, &U6),
                   F::from(choose(8, 4) * choose(15, 2)) / F::from(U6.len()));
    }
}

