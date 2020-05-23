#[cfg(test)]
mod tests {
    use quickfind::*;

    struct ConnectedRecord {
        left: usize,
        right: usize,
        is_connected: Option<bool>,
    }

    #[test]
    fn test_quick_find() {
        let mut quick_find = QuickFind::new(10);
        let union_commands = [(0, 5), (5, 6), (1, 2), (2, 7), (8, 3), (3, 4), (4, 9)];
        let connected_records = vec![
            ConnectedRecord {
                left: 5 as usize,
                right: 6 as usize,
                is_connected: Some(true),
            },
            ConnectedRecord {
                left: 0 as usize,
                right: 6 as usize,
                is_connected: Some(true),
            },
            ConnectedRecord {
                left: 1 as usize,
                right: 2 as usize,
                is_connected: Some(true),
            },
            ConnectedRecord {
                left: 2 as usize,
                right: 7 as usize,
                is_connected: Some(true),
            },
            ConnectedRecord {
                left: 1 as usize,
                right: 7 as usize,
                is_connected: Some(true),
            },
            ConnectedRecord {
                left: 3 as usize,
                right: 8 as usize,
                is_connected: Some(true),
            },
            ConnectedRecord {
                left: 3 as usize,
                right: 4 as usize,
                is_connected: Some(true),
            },
            ConnectedRecord {
                left: 4 as usize,
                right: 9 as usize,
                is_connected: Some(true),
            },
            ConnectedRecord {
                left: 3 as usize,
                right: 9 as usize,
                is_connected: Some(true),
            },
            ConnectedRecord {
                left: 8 as usize,
                right: 9 as usize,
                is_connected: Some(true),
            },
            ConnectedRecord {
                left: 0 as usize,
                right: 1 as usize,
                is_connected: None,
            },
            ConnectedRecord {
                left: 7 as usize,
                right: 8 as usize,
                is_connected: None,
            },
        ];

        for (left, right) in &union_commands {
            quick_find.union(*left, *right);
        }

        for i in connected_records {
            assert_eq!(quick_find.is_connected(i.left, i.right), i.is_connected);
        }
    }
}
