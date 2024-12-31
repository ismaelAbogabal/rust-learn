pub fn test() {
    //    println!("{}", int_to_roman(3749));
    // println!("{}", int_to_roman(58));
    println!("{}", int_to_roman(1994));
}

pub fn int_to_roman(num: i32) -> String {
    let digits = [
        ("M", 1000),
        ("CM", 900),
        ("D", 500),
        ("CD", 400),
        ("C", 100),
        ("XC", 90),
        ("L", 50),
        ("XL", 40),
        ("X", 10),
        ("IX", 9),
        ("V", 5),
        ("IV", 4),
        ("I", 1),
    ];

    let mut num = num;
    let mut output = String::new();

    for (ch, mult) in digits {
        let count = num / mult;
        num = num % mult;

        output += &ch.repeat(count as usize);
    }

    output
}
