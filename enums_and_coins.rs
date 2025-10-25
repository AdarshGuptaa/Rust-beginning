fn main(){
    let c1 = Coin::Nickle;
    let c2 = Coin::Dime;
    let c3 = Coin::Quarter(String::from("Alabama"));

    let c1 = sort_coin(c1);
    let c2 = sort_coin(c2);
    let c3 = sort_coin(c3);

    println!("c3: {c3}");
    println!("c1 : {}\nc2: {}", c1, c2);

    let a: Option<i32> = Some(5);
    let a = plus_one(a);

    


}

enum Coin{
    Nickle,
    Dime,
    Quarter(String)
}

fn sort_coin(coin : Coin) -> u8{
    match coin{
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(s) =>{
            println!("Quater from the state of {s}");
            25
        }
    }
}

fn plus_one(x : Option<i32>) -> Option<i32>{
    match x{
        None => None,
        Some(a) => Some(a + 1)
    }
}

