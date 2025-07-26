usr std::io;
use rand::prelude::*;


fn main() {
 let guess_list = ["banana","mango","apple","orange","peach"];
 let mut rng = thread_rng();

 let index rng.gen_rng(0..guess_list.length());

 let random_fruit = guess_list[index];
println!("random fruit:",random_fruit);
 let input = String::new();

 match io::stdin().read_line(&mut input){

 }
} 
