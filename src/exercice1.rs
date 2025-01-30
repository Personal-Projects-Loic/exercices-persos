use std::fmt::Display;
use std::mem;

pub struct SomethingToDisplay<T> {
    pub test: T,
    pub comment: String,
}

impl<T: Display> SomethingToDisplay<T> {
    pub fn print(&self) {
        println!(
            "C'etait un test {}\nCommentaire: {}",
            &self.test, &self.comment
        )
    }
}

pub fn swap_i32(mut a: i32, mut b: i32) {
    mem::swap(&mut a, &mut b);
    println!("a = {}, b = {}", a, b);
}

pub fn swap_any_type<T: Display>(a: &mut T, b: &mut T) {
    mem::swap(a, b);
    println!("a = {}, b = {}", a, b);
}
