use printpdf::*;
use std::fs::File;

fn main() {

    let file = File::open("./src/file.rs");

    match file {
        Ok(f) => println!("hello from print pdf {:?}", f),
        Err(_)=>  {match File::create("./src/file.rs"){
            Ok(cf)=> println!("{:?}", cf),
            Err(e)=> println!("{e}")
        }}
           
        
    }
   
}
