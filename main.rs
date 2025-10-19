mod ex2;

fn main() {

    let s1 : String = String:: from("Hello World!");

    println!("{s1}");

    let s2 = s1;

    println!("{s2}");

    let mut s3 = String:: from ("Ahoy!");

    s3 = s3 + " HAHA!";

    println!("{s3}");
    let (s3, len) = string_length(s3);

    println!("{len}");

    let len2 = string_length_better(&s2);

    println!("{len2}");

    let mut s4 = String::from("world");

    add_hello(&mut s4);

    println!("{s4}");
    
}

fn string_length(s : String) -> (String, u32) {

    let mut a : u32 = 0;
    for _x in s.chars(){
        a += 1;
    }

    (s, a)
}

fn string_length_better(s : &String) -> usize {
    let mut a : usize = 0;
    for _x in s.chars(){
        a += 1;
    }

    a
}

fn add_hello(s : &mut String){
    s.push_str(" hello")
}
