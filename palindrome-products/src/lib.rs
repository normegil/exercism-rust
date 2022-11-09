use std::collections::HashSet;

/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
        let is_palin;
        let dup ;

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
    let mut min_pal: Option<Palindrome> = Option::None;
    let mut max_pal: Option<Palindrome> = Option::None;
    for i in min..=max {
        for j in i..=max {
            let product = i*j;
            
            if min_pal == Option::None || min_pal.unwrap().into_inner() > product {
                if let Option::Some(pal) = Palindrome::new(product) {
                    min_pal = Option::Some(pal);
                }
            }
            if max_pal == Option::None || max_pal.unwrap().into_inner() < product {
                if let Option::Some(pal) = Palindrome::new(product) {
                    max_pal = Option::Some(pal);
                }
            }
        }
    }

    if min_pal.is_none() {
        return Option::None;
    } else {
        return Option::Some((min_pal.unwrap(), max_pal.unwrap()));
    }
}
