pub mod rever_word {
    use core::fmt;
    use std::fmt::format;

    pub fn test() {}

    pub fn reverse_words(s: String) -> String {
        let mut out = "".to_string();

        let splitted = s.split(" ");

        for word in splitted {
            if word != " " {
                out += " ";
                out += word;
            }
        }

        out
    }
}
