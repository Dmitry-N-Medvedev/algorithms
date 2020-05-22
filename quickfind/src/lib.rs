#[derive(Debug)]
pub struct QuickFind {
    items: Vec<u64>,
}

impl QuickFind {
    pub fn new(length: u64) -> QuickFind {
        let mut result: QuickFind = QuickFind { items: Vec::new() };

        for i in 0..length {
            result.items.push(i);
        }

        result
    }

    pub fn union(&mut self, left_index: u64, right_index: u64) {
        let left_group_id = self.items[left_index as usize];

        self.items[right_index as usize] = left_group_id;
    }

    pub fn get_items(&self) -> &[u64] {
        &self.items
    }

    pub fn is_connected(&self, left_index: u64, right_index: u64) -> bool {
        self.items[left_index as usize] == self.items[right_index as usize]
    }
}
