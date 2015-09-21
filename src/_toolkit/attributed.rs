use std::slice::Iter;

#[derive(Debug)]
struct SizedAttr<A> {
    size: usize,
    attr: A
}

#[derive(Debug)]
pub struct AttrFlow<A> {
    data: Vec<SizedAttr<A>>
}

impl<A> AttrFlow<A> {
    // def __getitem__(self, s):
    // def __setitem__(self, s, data):
    // def __add__(self, other):
    // def extend(self, data):
    fn new() -> AttrFlow<A> {
        AttrFlow { data: Vec::new() }
    }

    fn fill(size: usize, attr: A) -> AttrFlow<A> {
        AttrFlow { data: vec![SizedAttr { size: size, attr: attr }] }
    }

    fn len(&self) -> usize {
        let mut sum = 0;
        for a in self.data.iter() {
            sum += a.size;
        }
        sum
    }

    fn push(&mut self, size: usize, attr: A) {
        self.data.push(SizedAttr { size: size, attr: attr })
    }

    fn iter(&self) -> Iter<SizedAttr<A>> {
        self.data.iter()
    }
}

// impl fmt::Display for Vector2D {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "({}, {})", self.x, self.y)
//     }
// }

pub struct AttrString<A> {
    s: String,
    flow: AttrFlow<A>
}

impl<A> AttrString<A> {

    pub fn new(s: String, attr: A) -> AttrString<A> {
        let flow = AttrFlow::fill(s.len(), attr);
        AttrString {s: s, flow: flow}
    }

    pub fn fill(size: usize, c: char, attr: A) -> AttrString<A> {
        let mut s = String::with_capacity(size);
        for _ in 0..size {
            s.push(c)
        }
        AttrString::new(s, attr)
    }
    // def __init__(self, str=None, flow=None, attr=None):
    // def __len__(self):
    // def __iter__(self):
    // def __repr__(self):
    // def __getitem__(self, s):
    // def __setitem__(self, s, data):
    // def __add__(self, other):
}
