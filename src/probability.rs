use std::collections::HashSet;
use std::hash::Hash;

type F = fraction::Fraction;


/// "The probability of an event, given a sample space of equiprobable outcomes."
pub fn P<T: Eq + Hash>(event: HashSet<T>, space: HashSet<T>) -> F {
    F::from(event.intersection(&space).collect::<HashSet<_>>().len()) / F::from(space.len())
}


/// "The set of ways of concatenating one item from collection A with one from B."
pub fn cross (A: String, B: String) -> HashSet<String> {
    let mut set = HashSet::new();
    for &a in A.as_bytes() {
        for &b in B.as_bytes() {
            set.insert(String::from_utf8([a, b].to_vec()).unwrap());
        }
    }
    set
}



#[cfg(test)]
mod tests {
    use std::ops::Deref;
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

    #[test]
    fn urn_contents() {
        let set_w = cross("W".to_string(), "12345678".to_string());
        let set_b = cross("B".to_string(), "123456".to_string());
        let set_r = cross("R".to_string(), "123456789".to_string());
        let binaries = vec![set_w, set_b, set_r];
        let intersect = binaries
            .iter()
            .fold(None, |acc: Option<HashSet<&str>>, hs| {
                let hs = hs.iter().map(|s| s.deref()).collect();
                acc.map(|a| a.union(&hs).map(|s| *s).collect())
                    .or(Some(hs))
            });
        println!("{:?}", intersect);
    }

}

