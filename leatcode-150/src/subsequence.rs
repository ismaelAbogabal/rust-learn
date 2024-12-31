pub fn test() {
    println!(
        "{}",
        is_subsequence("aac".to_string(), "ahbgdc".to_string())
    );
}

pub fn is_subsequence(s: String, t: String) -> bool {
    let mut i = 0;
    let mut j = 0;

    let s = s.chars().collect::<Vec<char>>();
    let t = t.chars().collect::<Vec<char>>();

    while i < s.len() {
        loop {
            if j == t.len() {
                return false;
            } else if s[i] == t[j] {
                j += 1;
                break;
            }

            j += 1;
        }

        i += 1;
    }

    true
    //
}
