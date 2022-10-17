



## 掷骰子

用一个六面公平的骰子掷出偶数的概率是多少？

我们可以定义样本空间D和事件偶数，并计算出概率。


```rust
type F = fraction::Fraction;

pub fn P<T: Eq + Hash>(event: HashSet<T>, space: HashSet<T>) -> F {
    F::from(event.intersection(&space).collect::<HashSet<_>>().len()) / F::from(space.len())
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