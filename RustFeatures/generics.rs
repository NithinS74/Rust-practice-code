fn main() {
    find_largest(5,6);
    let p:Person = Person{
        name: String::from("name1"),
        age: 15,
    };
    println!("{}",P.describe());
    println!("{}",longest("Hello","Worlds"));
    let gs:Pair<i32> =  Pair::new(1,2);
    println!("{:#?}",gs);
}

fn find_largest<T:PartialOrd>(a:T,b:T)->T{
    if a>b{
        return a;
    }
    b
}

trait Describe{
    fn describe(&self)->String;
}

struct Person{
    name: String,
    age: u32,
}

impl Describe for Person{
    fn describe(&self)->String{
        String::from(format!("name : {}\nage: {}",self.name,self.age))
    }
}

fn longest<'a>(a: &'a str,b:&'a str)->&'a str{
    if a.len() > b.len(){
        return a;
    }
    b
}

#[derive(Debug)]
struct Pair<T>{
    a:T,
    b:T,
}

impl<T> Pair<T>{
    fn new(a:T,b:T)->Self{
        Self{
            a,
            b,
        }
    }
}
