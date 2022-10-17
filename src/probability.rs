use std::collections::HashSet;
use std::hash::Hash;

type F = fraction::Fraction;

pub fn P<T: Eq + Hash>(event: HashSet<T>, space: HashSet<T>) -> F {
    F::from(event.intersection(&space).collect::<HashSet<_>>().len()) / F::from(space.len())
}




#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn even() {
        use std::collections::HashSet;
        let D: HashSet<i32> =    vec![1, 2, 3, 4, 5, 6].into_iter().collect();
        let even: HashSet<i32> = vec![   2,    4,    6].into_iter().collect();
        println!("{:?}", P(even, D));
    }
}

