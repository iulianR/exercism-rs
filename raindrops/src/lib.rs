pub fn raindrops(n: u32) -> String {
    struct R {
        number: u32,
        sound: String,
    }

    let r = [
        R {
            number: 3,
            sound: "Pling".to_owned(),
        },
        R {
            number: 5,
            sound: "Plang".to_owned(),
        },
        R {
            number: 7,
            sound: "Plong".to_owned(),
        },
    ];

    let mut s = String::new();
    let res = r
        .iter()
        .filter(|r| n % r.number == 0)
        .map(|r| s.push_str(&r.sound))
        .collect::<Vec<_>>();

    if res.len() > 0 {
        s
    } else {
        n.to_string()
    }
}
