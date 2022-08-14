use std::collections::BTreeMap;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let uid = args[1].parse::<u32>().unwrap();

    for (name, key) in decode(uid) {
        println!("Unlock for {name}: {:?}", key);
    }
}

fn decode(uid: u32) -> Vec<(&'static str, Vec<char>)> {
    let consts = BTreeMap::<&str, u32>::from([
        // 1000297c
        ("Alice", 0xa5fa3b7f),
        // 10002980
        ("Bob", 0xe35c2742),
        // 10002984
        ("Carol", 0xbec5ca0f),
        // 10002988
        ("Dan", 0x87e35d46),
        // 1000298c
        ("Even", 0x5acd14f9),
        // 10002990
        ("Trevor", 0xabde1fcf),
    ]);

    let mut ret = vec![];

    // FUN_100026c0
    for (name, value) in consts {
        let key = value ^ uid;
        let mut final_key: Vec<char> = key.to_string().chars().collect();
        final_key.rotate_right(1);
        ret.push((name, final_key));
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let expected = vec![
            (
                "Alice",
                vec!['8', '2', '1', '2', '7', '7', '3', '7', '1', '5'],
            ),
            ("Bob", vec!['3', '9', '4', '7', '1', '7', '0', '6', '8']),
            (
                "Carol",
                vec!['6', '1', '7', '1', '0', '0', '4', '7', '2', '8'],
            ),
            (
                "Dan",
                vec!['7', '1', '5', '5', '6', '8', '6', '2', '8', '4'],
            ),
            (
                "Even",
                vec!['8', '2', '1', '7', '9', '3', '0', '9', '2', '4'],
            ),
            (
                "Trevor",
                vec!['8', '1', '8', '9', '5', '2', '0', '8', '4', '3'],
            ),
        ];

        let ret = decode(3676867129);
        assert_eq!(ret, expected);
    }
}
