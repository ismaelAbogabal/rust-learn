use std::collections::VecDeque;

pub fn test() {
    println!("{}", is_palindrome("0p".to_string()));
}

pub fn is_palindrome(s: String) -> bool {
    let mut chars: VecDeque<u8> = s
        .to_lowercase()
        .bytes()
        .filter(|char| {
            let char = *char;

            (122 >= char && char >= 97) || (57 >= char && char >= 48)
        })
        .collect();

    let mut left = chars.pop_back();
    let mut right = chars.pop_front();

    while left != None && right != None {
        let l = left.unwrap();
        let r = right.unwrap();

        if l != r {
            return false;
        } else {
            left = chars.pop_back();
            right = chars.pop_front();
        }
        // if both equal we continue
        // if one is invalid char continue
        // if there are a change we pause
    }

    true
}
