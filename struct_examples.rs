fn main(){

    let mut a1 = Animal{
        mammal : true,
        legs : 4,
        name : String::from("Bob"),
    };

    //got leg transplant
    a1.legs += 2;

    let a2 = build_animal(8, String::from("Peter"));

    let mut p1 = Point(1, 1, 1);
    let c1 = Color(101, 26, 222);

    let Point(ref x,ref y,ref z) = p1;

    println!("p1: {x} {y} {z}");

    p1.0 += 10;
    p1.1 += 11;

    let Point(ref x,ref y,ref z) = p1;

    println!("p1: {x} {y} {z}");

    println!("{a2:?}");

    dbg!(&a2);



    



}

#[derive(Debug)]
struct Animal{
    mammal : bool,
    legs : usize,
    name : String,
}

fn build_animal(legs : usize, name : String) -> Animal{
    let mut m : bool = false;

    if legs == 4{
        m = true;
    }

    let a1 = Animal{
        mammal : m,
        legs : legs,
        name : name,
    };

    a1
}


struct Color(i32, i32, i32);
struct Point(i32, i32, i32);