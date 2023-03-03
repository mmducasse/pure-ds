use std::fmt::{Debug, Display};

use crate::list::List;

#[derive(Clone)]
pub struct Queue<T>
where
    T: Clone,
{
    ins: List<T>,
    outs: List<T>,
}

impl<T> Queue<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        Self {
            ins: List::empty(),
            outs: List::empty(),
        }
    }

    pub fn from_slice(slice: &[T]) -> Self {
        Self {
            ins: List::from_slice(slice).reverse(),
            outs: List::empty(),
        }
    }

    fn revolve(&self) -> Self {
        Queue {
            ins: List::empty(),
            outs: self.outs.concat(&self.ins.reverse()),
        }
    }

    pub fn enqueue(&self, value: T) -> Self {
        Self {
            ins: self.ins.prepend(value),
            outs: self.outs.clone(),
        }
    }

    pub fn dequeue(&self) -> Option<(T, Self)> {
        let mut queue = self.clone();

        if queue.outs.is_empty() {
            queue = queue.revolve();
        }

        if let Some((value, rest)) = queue.outs.uncons() {
            let queue = Queue {
                ins: List::empty(),
                outs: rest,
            };

            Some((value, queue))
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<T> {
        let mut queue = self.clone();

        if queue.outs.is_empty() {
            queue = queue.revolve();
        }

        queue.outs.head()
    }
}

impl<T> Queue<T>
where
    T: Clone + Debug,
{
    pub fn into_string(&self) -> String {
        format!("{}{}", self.ins, self.outs.reverse())
    }
}

impl<T> Display for Queue<T>
where
    T: Clone + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Queue{}", self.into_string())
    }
}

impl<T> PartialEq for Queue<T>
where
    T: PartialEq + Clone,
{
    fn eq(&self, other: &Self) -> bool {
        (self.ins == other.ins) && (self.outs == other.outs)
    }
}
