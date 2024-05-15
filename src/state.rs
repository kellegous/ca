pub trait Apply {
    fn apply(&self, a: u8, b: u8, c: u8) -> u8;
}

#[derive(Clone, Debug)]
pub struct State {
    v: Vec<u8>,
}

impl State {
    pub fn with_size(n: usize) -> Self {
        Self { v: vec![0; n] }
    }

    pub fn get(&self, i: i32) -> u8 {
        self.v[i.rem_euclid(self.v.len() as i32) as usize]
    }

    pub fn len(&self) -> usize {
        self.v.len()
    }

    pub fn is_empty(&self) -> bool {
        self.v.is_empty()
    }

    pub fn set(&mut self, i: i32, value: u8) {
        let n = self.len() as i32;
        self.v[i.rem_euclid(n) as usize] = value;
    }

    pub fn apply_into(&self, rule: &dyn Apply, next: &mut State) {
        for i in 0..self.len() {
            let i = i as i32;
            next.set(i, rule.apply(self.get(i - 1), self.get(i), self.get(i + 1)));
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = u8> + '_ {
        self.v.iter().copied()
    }

    pub fn apply(&self, rule: &dyn Apply) -> State {
        let mut next = State::with_size(self.len());
        self.apply_into(rule, &mut next);
        next
    }
}
