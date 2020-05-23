#[derive(Debug)]
pub struct QuickFind {
    items: Vec<u64>,
}

impl QuickFind {
    pub fn new(length: u64) -> QuickFind {
        let mut result = QuickFind {
            items: Vec::with_capacity(length as usize),
        };

        for i in 0..length {
            result.items.push(i);
        }

        result
    }

    pub fn union(&mut self, left_index: usize, right_index: usize) {
        let left_group_id = self.items.get(left_index);

        match left_group_id {
            Some(id) => self.items[right_index] = *id,
            None => {}
        }
    }

    pub fn items(&self) -> &[u64] {
        &self.items
    }

    pub fn is_connected(&self, left_index: usize, right_index: usize) -> Option<bool> {
        let left = self.items.get(left_index);

        match left.is_some() && left == self.items.get(right_index) {
            true => Some(true),
            false => None,
        }
    }
}
