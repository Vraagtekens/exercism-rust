// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

use std::fmt::Display;

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<T>(std::marker::PhantomData<T>);

impl<T> Matcher<T> {
    pub fn new<F, S>(_matcher: F, _subs: S) -> Matcher<T> {
        todo!()
    }
}

///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
pub struct Fizzy<T>(std::marker::PhantomData<T>);

impl<T> Fizzy<T> {
    pub fn new() -> Self {
        return Self(std::marker::PhantomData);
    }

    // feel free to change the signature to `mut self` if you like
    #[must_use]
    pub fn add_matcher(self, _matcher: Matcher<T>) -> Self {
        todo!()
    }

    pub fn apply<I>(self, iter: I) -> impl Iterator<Item = String>
    where
        I: Iterator,
        I::Item: Into<i32> + Display + Copy,
    {
        iter.map(|x| {
            let num: i32 = x.into();

            if num % 3 == 0 && num % 5 == 0 {
                "fizzbuzz".to_string()
            } else if num % 3 == 0 {
                "fizz".to_string()
            } else if num % 5 == 0 {
                "buzz".to_string()
            } else {
                x.to_string()
            }
        })
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T>() -> Fizzy<T> {
    Fizzy::new()
}
