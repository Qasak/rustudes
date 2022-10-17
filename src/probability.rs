use std::collections::HashSet;
use std::hash::Hash;
use itertools::Itertools;
use std::str;
use rand::seq::index;
type F = fraction::Fraction;


/// "The probability of an event, given a sample space of equiprobable outcomes."
pub fn P <T: Eq + Hash>(event: HashSet<T>, space: HashSet<T>) -> F {
    F::from(event.intersection(&space).collect::<HashSet<_>>().len()) / F::from(space.len())
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
pub fn combos (items: Vec<String>, n: usize) -> Vec<Vec<String>> {
    let it = items.into_iter().combinations(n).collect::<Vec<_>>();
    it
}



#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use rand::seq::IteratorRandom;
    use rand::thread_rng;
    use super::*;
    #[test]
    fn even() {
        let D: HashSet<i32> =    vec![1, 2, 3, 4, 5, 6].into_iter().collect();
        let even: HashSet<i32> = vec![   2,    4,    6].into_iter().collect();
        println!("{:?}", P(even, D));
    }

    #[test]
    fn even_2() {
        let D: HashSet<i32> =    vec![1, 2, 3, 4, 5, 6].into_iter().collect();
        let even: HashSet<i32> = vec![2, 4, 6, 8, 10, 12].into_iter().collect();
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
        // let urn = get_urn();
        // let U6 = combos(urn, 6);
        // let red6 = U6.iter().
        //     map(|i| i.iter().map(|j| j.as_bytes().iter().filter(|b| &&b == 'R')))
    }

}

