// Make programme leaper year using match function
fn main() {
    let num=2001;
    match (num%400, num%100, num%4 ) {
        (0,_,_) => println!("This is leaper year"),
        (_,0,_) => println!("This is not leaper year"),
        (_,_,0)=> println!("This is leaper year"),
        _ => println!("This is not leaper year"),
    }
}
