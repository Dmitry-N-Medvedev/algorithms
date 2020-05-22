#[cfg(test)]
mod tests {
    use quickfind::*;

    struct ConnectedRecord {
        left: usize,
        right: usize,
        is_connected: bool,
    }

    #[test]
    fn test_quick_find() {
        let mut quick_find = QuickFind::new(100);
        let union_commands = [0, 5, 5, 6, 1, 2, 2, 7, 8, 3, 3, 4, 4, 9];
        let connected_records = vec![
            ConnectedRecord {
                left: 5,
                right: 6,
                is_connected: true,
            },
            ConnectedRecord {
                left: 0,
                right: 6,
                is_connected: true,
            },
            ConnectedRecord {
                left: 1,
                right: 2,
                is_connected: true,
            },
            ConnectedRecord {
                left: 2,
                right: 7,
                is_connected: true,
            },
            ConnectedRecord {
                left: 1,
                right: 7,
                is_connected: true,
            },
            ConnectedRecord {
                left: 3,
                right: 8,
                is_connected: true,
            },
            ConnectedRecord {
                left: 3,
                right: 4,
                is_connected: true,
            },
            ConnectedRecord {
                left: 4,
                right: 9,
                is_connected: true,
            },
            ConnectedRecord {
                left: 3,
                right: 9,
                is_connected: true,
            },
            ConnectedRecord {
                left: 8,
                right: 9,
                is_connected: true,
            },
            ConnectedRecord {
                left: 0,
                right: 1,
                is_connected: false,
            },
            ConnectedRecord {
                left: 7,
                right: 8,
                is_connected: false,
            },
        ];

        let mut ptr = 0;

        while ptr < union_commands.len() {
            let left = union_commands[ptr];
            let right = union_commands[ptr + 1];

            quick_find.union(left, right);

            ptr += 2;
        }

        for i in connected_records {
            assert_eq!(quick_find.is_connected(i.left, i.right), i.is_connected);
        }
    }
}
