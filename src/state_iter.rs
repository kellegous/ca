use crate::{Apply, State};

pub struct StateIter<T: Apply> {
    rule: T,
    next: State,
}

impl<T: Apply> StateIter<T> {
    pub fn new(rule: T, state: State) -> Self {
        Self { rule, next: state }
    }
}

impl<T: Apply> Iterator for StateIter<T> {
    type Item = State;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.next.clone();
        self.next = curr.apply(&self.rule);
        Some(curr)
    }
}
