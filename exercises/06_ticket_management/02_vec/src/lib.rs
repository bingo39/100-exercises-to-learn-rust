// Given a number `n`, return the `n+1`th number in the Fibonacci sequence.
//
// The Fibonacci sequence is defined as follows:
//
// - The first number of the sequence is 0.
// - The second number of the sequence is 1.
// - Every subsequent number is the sum of the two preceding numbers.
//
// So the sequence goes: 0, 1, 1, 2, 3, 5, 8, 13, 21, and so on.
//
// We expect `fibonacci(0)` to return `0`, `fibonacci(1)` to return `1`,
// `fibonacci(2)` to return `1`, and so on.

use core::num;

/*
 给定一个数字 `n`，返回斐波那契数列中第 `n+1` 个数。

 斐波那契数列定义如下：

 - 数列的第一个数是 0。
 - 数列的第二个数是 1。
 - 之后每个数都是前两个数之和。

 因此数列为：0, 1, 1, 2, 3, 5, 8, 13, 21，以此类推。

 我们期望 `fibonacci(0)` 返回 `0`，`fibonacci(1)` 返回 `1`，
 `fibonacci(2)` 返回 `1`，以此类推。 */
pub fn fibonacci(n: u32) -> u32 {
    // TODO: implement the `fibonacci` function
    //
    // Hint: use a `Vec` to memoize the results you have already calculated
    // so that you don't have to recalculate them several times.
    let mut number:Vec<u32> = Vec::new();
    number.push(0);
    let mut j = 0;
    for i in 1..=n{
        // u32是内存空间占用，无法作为索引；需要转换为usize
        let mut size = i as usize;
        if size > 1 {
           j = number[size-1]+number[size-2];
        }else{
            j = 1 ;
        }
        number.push(j);
    }
    return j;
}

#[cfg(test)]
mod tests {
    use std::print;

use crate::fibonacci;

    #[test]
    fn first() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn second() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(fibonacci(2), 1);
    }

    #[test]
    fn tenth() {
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn thirtieth() {
        assert_eq!(fibonacci(30), 832040);
    }
}
