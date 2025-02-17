#[derive(Debug)]
struct book{
	title: String,
	author: String,
	pages: u32,
}

impl book{
	fn reading_time_needed(&self){
		println!("no of hours needed to read : {}", self.pages / 30);
	}
}

fn main(){
	let x = book{
		title: String::from("lord of the rings"),
		author: String::from("tolken"),
		pages: 345
};

x.reading_time_needed();

println!("{:?}",x);
}
