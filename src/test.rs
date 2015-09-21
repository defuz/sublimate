use std::cmp;
use std::convert::AsRef;
use std::iter::Iterator;

trait HasWidth<K: ?Sized> {
    fn width(&self, context: K) -> usize;
}

struct Button;

impl<K> HasWidth<K> for Button where K: AsRef<str> {
    fn width(&self, context: K) -> usize {
        context.as_ref().len()
    }
}

struct Decorator<I>(u8, I);

impl<K, I> HasWidth<K> for Decorator<I> where I: HasWidth<K> {
    fn width(&self, context: K) -> usize {
        match self {
            &Decorator(_, ref item) => item.width(context)
        }
    }
}

struct Selected<I>(u8, I);

impl<K, I> HasWidth<(K, bool)> for Selected<I> where I: HasWidth<K> {
    fn width(&self, (context, focused): (K, bool)) -> usize {
        match self {
            &Selected(_, ref item) => item.width(context)
        }
    }
}

struct VerticalRender<I>(I);

impl<K, I, R> HasWidth<R> for VerticalRender<I> where I: HasWidth<K>, R: Iterator<Item=K> {
    fn width(&self, context: R) -> usize {
        match self {
            &VerticalRender(ref item) => {
                let mut w = 0;
                for c in context {
                    w = cmp::max(w, item.width(c));
                }
                w
            }
        }
    }
}

fn main() {
    let view = Decorator(6, VerticalRender(Selected(7, Button)));
    let buttons = ["hello".to_string(), "world".to_string()];
    let width = view.width(buttons.iter().map(|s| (s, true)));

    println!("{}", width);

}
