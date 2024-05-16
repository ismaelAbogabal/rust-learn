

// [This code does not compile!] 
fn main() {
    let s = String::from("Hello world");

    let a = first_word(&s);

    println!("{}", a);

    let b = Box::new(s);


    use_myBox(  &b);
    use_myBox(  &b);

}


fn use_myBox(b: &Box<String>) {
    println!("{}", b);
}


fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}