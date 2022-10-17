



## 掷骰子

用一个六面公平的骰子掷出偶数的概率是多少？

我们可以定义样本空间D和事件偶数，并计算出概率。


```rust
///     """The probability of an event, given a sample space of equiprobable outcomes.
//     event can be either a set of outcomes, or a predicate (true for outcomes in the event)."""
pub fn P <T: Eq + Hash>(event: &Vec<T>, space: &Vec<T>) -> F {
    let event_set: HashSet<&T> = event.into_iter().collect();
    let space_set: HashSet<&T> = space.into_iter().collect();
    let l1 = event_set.intersection(&space_set).collect::<HashSet<_>>().len();
    let l2 = space_set.len();
    F::from(l1) / F::from(l2)
}
```
为什么P的定义使用len（event & space）而不是len（event）？因为我不想计算那些在事件中被指定但实际上不在样本空间中的结果

```rust
#[cfg(test)]
mod tests {
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

}
```


我们将从定义瓮的内容开始。
一个瓮里有23个球。8个白色，6个蓝色，9个红色。我们随机选择六个球（每种可能的选择都是同样的可能性）。考虑如下事件的概率

+ 所有的球都是红色的
+ 3个是蓝色的，2个是白色的，1个是红色的
+ 恰好有4个球是白色的


```rust
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

#[test]
fn get_urn() {
    let W = cross("W", "12345678");
    let B = cross("B", "123456");
    let R = cross("R", "123456789");
    let urn = vec![W, B, R].into_iter().flatten().collect::<Vec<_>>();
    println!("{:?}", urn);
}

```

```rust
Some({"R5", "R9", "W1", "W2", "W4", "R7", "B4", "B2", "W3", "R2", "R1", "W7", "R8", "R6", "R4", "B3", "W8", "B6", "R3", "W5", "W6", "B1", "B5"})
```

现在我们可以将样本空间U6定义为所有6球组合的集合。我们使用itertools.combinations来生成组合，然后将每个组合连接成一个字符串。

```rust
/// "All combinations of n items; each combo as a concatenated str."
pub fn combos (items: Vec<String>, n: usize) -> Vec<Vec<String>> {
    let it = items.into_iter().combinations(n).collect::<Vec<_>>();
    it
}

```

```rust
#[test]
fn get_combos() {
    let urn = get_urn();
    let U6 = combos(urn, 6);
    let mut rng = thread_rng();
    let sample = U6.iter().choose_multiple(&mut rng, 10);
    println!("{:?}", U6.len());
    println!("{:?}", sample);
}

100947
[["W3", "W5", "B2", "B3", "B4", "B5"], ["W3", "W4", "W8", "R1", "R3", "R9"], ["W1", "W5", "W7", "W8", "B6", "R7"], ["W3", "W4", "W6", "B1", "R2", "R3"], ["W1", "W4", "W5", "B3", "R5", "R8"], ["W6", "W8", "B1", "B4", "R4", "R7"], ["W1", "W3", "B5", "B6", "R3", "R6"], ["W6", "B2", "B4", "R1", "R3", "R8"], ["W2", "W3", "B2", "B4", "B5", "R6"], ["W2", "W5", "B1", "B2", "B6", "R2"]]

```
23 choose 6 = $\frac{23⋅22⋅21⋅20⋅19⋅18}{6!}=100947$

n choose c  = $\frac{n!}{(n - c)!·c!}$


### 瓮问题1：选择6个红球的概率是多少？

```rust
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

red6.len(): 84
Rational(Plus, Ratio { numer: 4, denom: 4807 })


```

为什么会有84种方法？因为瓮里有9个红球，而我们问的是有多少种方法可以选择其中的6个。

`choose(9, 6)`

84

```rust
        assert_eq!(P(&red6, &U6), F::from(choose(9, 6)) / F::from(U6.len()));
```
所有涉及代码：

[probability.rs](https://github.com/Qasak/rustudes/blob/main/src/probability.rs)