pub mod rever_word {

    pub fn test() {
        println!("{} ", reverse_words("Hello world".to_string()));
    }

    pub fn reverse_words(s: String) -> String {
        let mut out = "".to_string();

        // out.push_str(string);
        let splitted = s.split(" ");

        for word in splitted {
            if word != "".to_string() {
                out = format!("{} {}", word, out)
            }
        }

        out.trim().to_string()
    }
}
