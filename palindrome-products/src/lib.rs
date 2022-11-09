/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        match Palindrome::is_palindrome(value, value) {
            (_, true) => Option::Some(Palindrome(value)),
            (_, false) => Option::None,
        }
    }

    fn is_palindrome(val: u64, dup_val:u64) -> (u64, bool) {
        let mut is_palin = true;
        let mut dup = dup_val;

        if val == 0 {
            return (dup_val, true);
        } else {
            (dup, is_palin) = Palindrome::is_palindrome(val / 10, dup_val);
        }

        if !is_palin {
            return (0, false);
        } else if val % 10 == dup % 10 {
            return (dup / 10, is_palin)
        } else {
            return (0, false);
        }
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    unimplemented!(
        "returns the minimum and maximum number of palindromes of the products of two factors in the range {} to {}",
        min, max
    );
}
