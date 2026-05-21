// TODO: Define a new trait, `Power`, that has a method `power` that raises `self`
//  to the power of `n`.
//  The trait definition and its implementations should be enough to get
//  the tests to compile and pass.
//
// Recommendation: you may be tempted to write a generic implementation to handle
// all cases at once. However, this is fairly complicated and requires the use of
// additional crates (i.e. `num-traits`).
// Even then, it might be preferable to use a simple macro instead to avoid
// the complexity of a highly generic implementation. Check out the
// "Little book of Rust macros" (https://veykril.github.io/tlborm/) if you're
// interested in learning more about it.
// You don't have to though: it's perfectly okay to write three separate
// implementations manually. Venture further only if you're curious.

use std::result;

/**
 * 建议：您可能会试图编写一个通用实现来一次性处理所有情况。
 * 然而，这相当复杂，需要使用额外的板条箱（即`num traits`）。
 * 即使如此，最好还是使用简单的宏，以避免高度通用实现的复杂性。
 * 如果您有兴趣了解更多关于它的信息,查看“Rust宏小书”(https://veykril.github.io/tlborm/)。
 * 否则，您不必这样做：手动编写三个单独的实现是完全可以的。只有当你好奇的时候，才去冒险。
 */

pub trait Power<Exponent> {
    type Output;
    fn power(self,exp:Exponent) -> Self::Output;
}
impl Power<u16> for u32 {
    type Output = u32;
    fn power(self,exp:u16) -> Self::Output {
        let mut result = 1u32;
        for _ in 0..exp{
            result *= self;
        }
        result
    }
}
impl Power<u32> for u32 {
    type Output = u32;
    fn power(self,exp:u32) -> Self::Output {
        self.pow(exp)
    }
}
impl Power<&u32> for u32{
    type Output = u32;
    fn power(self,exp:&u32) -> Self::Output {
        let mut result = 1u32;
        for _ in 0..*exp{
            result *= self;
        }
        result
    }
}





#[cfg(test)]
mod tests {
    use super::Power;

    #[test]
    fn test_power_u16() {
        let x: u32 = 2_u32.power(3u16);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_u32() {
        let x: u32 = 2_u32.power(3u32);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_ref_u32() {
        let x: u32 = 2_u32.power(&3u32);
        assert_eq!(x, 8);
    }
}
