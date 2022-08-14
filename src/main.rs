use std::collections::BTreeMap;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let uid = args[1].parse::<u32>().unwrap();

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

    // FUN_100026c0
    for (name, value) in consts {
        let key = value ^ uid;
        let mut final_key: Vec<char> = key.to_string().chars().collect();
        final_key.rotate_right(1);
        println!("Unlock for {name}: {:?}", &final_key);
    }
}
