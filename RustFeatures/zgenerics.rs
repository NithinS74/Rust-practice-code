#![allow(warnings)]
fn main(){

}

fn compare_and_return<T>(a:T,b:T)->T 
    where T: PartialOrd+Debug
{
   if a<b{
        return a;
    }    
    b
}

trait Shape{
    fn area(&self)->f64;
}

struct Rectangle{
    length:f62,
    width:f62,
}

impl Shape for Rectangle{
    fn area(&self)->f64{
        let area:f64 = self.length * self.width;
        area
    }
}

fn print_area<T:Shape>(shape:T){
    let area:f64 = shape.area();
    println!("area: {area}");
}

struct Text<'a>{
    string: &'a str,
}

