// cow1.rs

/*

在这个练习中，我们将探讨 Cow 类型，它是一个写时复制的智能指针。
Cow 可以包装和提供对借用数据的不可变访问，并在需要修改或拥有权时惰性地克隆数据。

为了修复这个练习中的单元测试，我们需要检查 Cow 类型的值是否是
 Cow::Owned(_) 或 Cow::Borrowed(_)。如果值是 Cow::Owned(_)，
 则表示它已经被克隆并拥有了所有权，可以修改它。
如果值是 Cow::Borrowed(_)，则表示它是一个借用值，不能被修改。
 */
use std::borrow::Cow;

fn abs_all<'a, 'b>(input: &'a mut Cow<'b, [i32]>) -> &'a mut Cow<'b, [i32]> {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // Clones into a vector if not already owned.
            // 使用了 Cow 类型的 to_mut 方法。to_mut 方法会将 Cow 类型的值转换为一个可变引用，如果值不是拥有所有权的，则会发生克隆。
            input.to_mut()[i] = -v;
        }
    }
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_mutation() -> Result<(), &'static str> {
        // Clone occurs because `input` needs to be mutated.
        let slice = [-1, 0, 1];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }

    #[test]
    fn reference_no_mutation() -> Result<(), &'static str> {
        // No clone occurs because `input` doesn't need to be mutated.
        let slice = [0, 1, 2];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            Cow::Borrowed(_) => Ok(()),
            _ => Err("Expected borrowed value"),
        }
    }

    #[test]
    fn owned_no_mutation() -> Result<(), &'static str> {
        // We can also pass `slice` without `&` so Cow owns it directly.
        // In this case no mutation occurs and thus also no clone,
        // but the result is still owned because it was never borrowed
        // or mutated.
        let slice = vec![0, 1, 2];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }

    #[test]
    fn owned_mutation() -> Result<(), &'static str> {
        // Of course this is also the case if a mutation does occur.
        // In this case the call to `to_mut()` returns a reference to
        // the same data as before.
        let slice = vec![-1, 0, 1];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }
}
