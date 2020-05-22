#[derive(Debug)]
pub struct QuickFind {
    items: Vec<u64>,
}

impl QuickFind {
    pub fn new(length: u64) -> QuickFind {
        QuickFind {
            items: (0..length as u64).collect(),
        }
    }

    pub fn union(&mut self, left_index: usize, right_index: usize) {
        let left_group_id = self.items[left_index];

        self.items[right_index] = left_group_id;
    }

    pub fn items(&self) -> &[u64] {
        &self.items
    }

    pub fn is_connected(&self, left_index: usize, right_index: usize) -> bool {
        self.items[left_index] == self.items[right_index]
    }
}
