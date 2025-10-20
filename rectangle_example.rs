struct Rectangle{
    width : usize,
    height: usize,
}

impl Rectangle{
    fn new(width : usize, height: usize) -> Self{
        Self { width, height }
    }

    fn area(&self)-> usize{
        self.height * self.width
    }

    fn fits_in(&self, rec : &Rectangle) -> bool{
        if rec.area() > self.area(){
            return true;
        }

        false
    }
}


fn main(){
    let rec1 = Rectangle::new(10, 5);
    let rec2 = Rectangle::new(100, 50);


    let area1 = rec1.area();

    let area2 = rec2.area();

    println!("Rec1: {}\nRec2: {}", area1, area2 );

    let fits_within = rec1.fits_in(&rec2);

    println!("Rec1 fits within Rec2: {}", fits_within);
}
