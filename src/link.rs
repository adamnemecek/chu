// //
// use std::rc::Rc;
// pub enum Link1 {
//     //
//     None,
//     Some(usize, Option<Rc<Self>>), // datum: usize,
//                                    // next: Option<std::rc::Rc<Self>>,
// }

// impl Clone for Link1 {
//     fn clone(&self) -> Self {
//         match self {
//             Self::None => Self::None,
//             Self::Some(v, r) => Self::Some(*v, r.clone()),
//         }
//     }
// }

// impl Link1 {
//     pub fn new(datum: usize, next: Option<Rc<Self>>) -> Rc<Self> {
//         Self::Some(datum, next.clone()).into()
//     }

//     pub fn from(mut i: impl Iterator<Item = usize>) -> Option<Self> {
//         // let mut m = Self::new

//         let Some(e) = i.next() else { return None };
//         // let head = Self::new(e, None)

//         unimplemented!()
//     }

//     pub fn datum(&self) -> Option<usize> {
//         match self {
//             Self::None => None,
//             Self::Some(v, _) => Some(*v),
//         }
//     }
// }

// impl Iterator for Link1 {
//     type Item = Rc<Self>;
//     fn next(&mut self) -> Option<Self::Item> {
//         match self {
//             Self::None => None,
//             Self::Some(v, n) => n.clone(),
//         }
//     }
// }

// fn test1() {
//     #[test]

//     fn test1() {
//         //
//     }
// }
// use std::collections::LinkedList;

// pub struct List<T>(LinkedList<T>);

// impl<T> List<T> {
//     pub fn new() -> Self {
//         Self(LinkedList::new())
//     }

//     pub fn prepend(&mut self, elt: T) {
//         self.0.push_front(elt)
//     }
// }

// impl<T> Iterator for List<T> {
//     fn next(&mut self) -> Option<Self::Item> {
//         self.0.nex
//     }
// }

//

pub trait VecExt<T> {
    fn from_arrays<const R: usize, const C: usize>(data: [[T; C]; R]) -> Self
    where
        T: Copy;

    fn fill(count: usize, f: impl Fn() -> T) -> Self;
}

impl<T> VecExt<T> for Vec<T> {
    fn from_arrays<const R: usize, const C: usize>(data: [[T; C]; R]) -> Self
    where
        T: Copy,
    {
        let mut self_ = Self::with_capacity(R * C);
        for e in data {
            self_.extend(e.iter());
        }
        self_
    }

    fn fill(count: usize, f: impl Fn() -> T) -> Self {
        let mut self_ = Self::with_capacity(count);
        for _ in 0..count {
            self_.push(f());
        }
        self_
    }
}

// pub struct Alloc<T>(Vec<T>);

// impl<T> Alloc<T> {
//     fn new() -> Self {
//         Self(vec![])
//     }

//     pub fn insert(&mut self, x: T) -> usize {
//         let i = self.0.len();
//         self.0.push(x);
//         i
//     }
// }
