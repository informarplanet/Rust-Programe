// Enum Example
enum color {
    red,
    green, 
    blue
}

fn print_color(my_color:color){
    match my_color {
        color::red => println!("The color is red"),
        color::green => println!("The color is green"),
        color::blue => println!("The color is blue"),
    }

}


fn main() {
    print_color(color::red);
}

