use std::{
    fmt::{Debug, Display},
    rc::Rc,
};

#[derive(Clone)]
pub struct List<T>
where
    T: Clone,
{
    inner: Rc<ListEnum<T>>,
}

#[derive(Clone)]
enum ListEnum<T>
where
    T: Clone,
{
    Node { value: T, next: List<T> },
    Empty,
}

impl<T> List<T>
where
    T: Clone,
{
    pub fn empty() -> Self {
        Self {
            inner: Rc::new(ListEnum::Empty),
        }
    }

    pub fn node(value: T, next: Self) -> Self {
        let node = ListEnum::Node { value, next };
        Self {
            inner: Rc::new(node),
        }
    }

    pub fn from_slice(slice: &[T]) -> Self {
        fn _from_slice<T>(slice: &[T], index: usize) -> List<T>
        where
            T: Clone,
        {
            if index >= slice.len() {
                List::empty()
            } else {
                let next = _from_slice(slice, index + 1);
                next.prepend(slice[index].clone())
            }
        }
        _from_slice(slice, 0)
    }

    fn inner(&self) -> &ListEnum<T> {
        self.inner.as_ref()
    }

    pub fn head(&self) -> Option<T> {
        match self.inner() {
            ListEnum::Node { value, .. } => Some(value.clone()),
            ListEnum::Empty => None,
        }
    }

    pub fn tail(&self) -> Option<Self> {
        match self.inner() {
            ListEnum::Node { value: _, next } => Some(next.clone()),
            ListEnum::Empty => None,
        }
    }

    pub fn prepend(&self, value: T) -> Self {
        self.insert(value, 0).unwrap()
    }

    pub fn insert(&self, value: T, index: usize) -> Option<Self> {
        if index == 0 {
            Some(Self::node(value, self.clone()))
        } else {
            match self.inner() {
                ListEnum::Node { value: v, next } => {
                    let next = next.insert(value, index - 1)?;
                    let new = Self::node(v.clone(), next);
                    Some(new)
                }
                ListEnum::Empty => None,
            }
        }
    }

    pub fn reverse(&self) -> Self {
        fn _reverse<T>(node: &List<T>, tail: List<T>) -> List<T>
        where
            T: Clone,
        {
            match node.inner() {
                ListEnum::Node { value, next } => {
                    let node = List::node(value.clone(), tail);
                    _reverse(next, node)
                }
                ListEnum::Empty => tail,
            }
        }
        _reverse(self, List::empty())
    }
}

impl<T> List<T>
where
    T: Clone + Debug,
{
    pub fn print(&self) {
        match self.inner() {
            ListEnum::Node { value, next } => {
                print!("{:?}, ", value);
                next.print();
            }
            ListEnum::Empty => {
                println!("_");
            }
        }
    }

    pub fn into_string(&self) -> String {
        match self.inner() {
            ListEnum::Node { value, next } => {
                let mut owned = format!("{:?}, ", value);
                let next = next.into_string();
                owned.push_str(&next);
                owned
            }
            ListEnum::Empty => "_".to_owned(),
        }
    }
}

impl<T> Display for List<T>
where
    T: Clone + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.into_string())
    }
}

impl<T> Debug for List<T>
where
    T: Clone + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?}]", self.into_string())
    }
}

impl<T> PartialEq for List<T>
where
    T: PartialEq + Clone,
{
    fn eq(&self, other: &Self) -> bool {
        use ListEnum::*;

        match (self.inner(), other.inner()) {
            (
                Node {
                    value: v1,
                    next: n1,
                },
                Node {
                    value: v2,
                    next: n2,
                },
            ) => (v1 == v2) && (n1 == n2),
            (Empty, Empty) => true,
            _ => false,
        }
    }
}
