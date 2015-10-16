
use std::cmp;
use std::borrow::Borrow;
use std::convert::AsRef;
use std::iter::Iterator;
use std::fmt::Debug;

trait Test<C> {
    fn test(&self, context: C);
}

#[derive(Debug)]
struct Item {
    x: u64,
}

struct Smart;

impl<I, C> Test<I> for Smart where I: Iterator<Item=C>, C: Debug {
    fn test(&self, context: I) {
        for c in context {
            println!("{:?}", c);
            println!("{:?}", c)
        }
    }
}


fn main() {
    let x = vec![Item { x: 5 }];
    let s = Smart;
    s.test(x.iter().map(|i| i));
}
