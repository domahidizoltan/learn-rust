use std::borrow::BorrowMut;

// FIX the error without removing any code line
#[derive(Debug)]
struct test {
    list: Vec<i32>,
    a: i32
}

impl test {
    pub fn new() -> Self {
        test { list:vec![1,2,3,4,5,6,7], a:0 }
    }

    pub fn run(&mut self) {
        for i in self.list.to_vec().iter() {
            self.do_something(*i)
        }

    }

    pub fn do_something(&mut self, n: i32) {
        self.a = n;
    }
}

pub fn main() {
    let mut t = test::new();
    t.run();
    println!("{:?}", t)
}
