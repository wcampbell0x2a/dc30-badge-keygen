use std::collections::BTreeMap;

const PHONE_NUMBER_DIGITS: usize = 10;
const TOTAL_NUMBERS: usize = 6;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let uid = args[1].parse::<u32>().unwrap();

    for (name, key) in decode(uid) {
        let first = key.iter().take(3).collect::<String>();
        let second = key.iter().skip(3).take(3).collect::<String>();
        let third = key.iter().skip(6).take(4).collect::<String>();
        println!("Unlock for {name:>6}: {first}-{second}-{third}");
    }
}

fn decode(uid: u32) -> [(&'static str, [char; PHONE_NUMBER_DIGITS]); TOTAL_NUMBERS] {
    let hashs = BTreeMap::<&str, u32>::from([
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

    // FUN_100026c0
    let mut ret = [("", [' '; PHONE_NUMBER_DIGITS]); TOTAL_NUMBERS];
    for (i, (name, value)) in hashs.iter().enumerate() {
        let key = value ^ uid;
        let mut final_key: Vec<char> = key.to_string().chars().collect();
        final_key.rotate_right(1);

        let mut phone_number = [' '; PHONE_NUMBER_DIGITS];
        for (i, num) in final_key.iter().enumerate() {
            phone_number[i] = *num;
        }
        ret[i] = (name, phone_number);
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let expected = [
            ("Alice", ['8', '2', '1', '2', '7', '7', '3', '7', '1', '5']),
            ("Bob", ['3', '9', '4', '7', '1', '7', '0', '6', '8', ' ']),
            ("Carol", ['6', '1', '7', '1', '0', '0', '4', '7', '2', '8']),
            ("Dan", ['7', '1', '5', '5', '6', '8', '6', '2', '8', '4']),
            ("Even", ['8', '2', '1', '7', '9', '3', '0', '9', '2', '4']),
            ("Trevor", ['8', '1', '8', '9', '5', '2', '0', '8', '4', '3']),
        ];

        let ret = decode(3676867129);
        assert_eq!(ret, expected);
    }
}
