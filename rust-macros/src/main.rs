use rust_macros::subtract;  //package-name::function (src/lib.rs)
use rust_macros::Addition;
use rust_macros::say_number;

#[derive(Addition)]                 //Derive Macro
struct Struct;

subtract!();   // ! , macro symbol  //Function Macro


#[say_number(Four)]
fn invoke_attr(){}

/** Declarative **/
macro_rules! times_five {
    ($e:expr) => { 5 * $e }
}

fn main(){
    println!("{}", diff());
    println!("{}", sum());
    println!("{}", times_five!(5));
}
