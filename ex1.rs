fn main() {
    let s = String::from("Hello World!");

    let word_index = get_word_index(&s);

    println!("{s} -> {word_index}");

    let a = &s[0..(s.len())].len();
    let b = &s[6..11].len();

    println!("{a}");
    println!("{b}");

    let word_slice = get_word_slice(&s);

    println!("{word_slice}");

}

fn get_word_index(s: &String) -> usize {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn get_word_slice(s : &String) -> &str{
    for (i, &el) in s.as_bytes().iter().enumerate(){
        if el == b' '{
            return &s[..i];
    }

    
}

return &s[..];
}
