#![allow(unused_variables)]

fn main() {
    //tuple
    let location: (&str, f64, f64) = ("KCLE", 41.4094069, -81.8546911);

    // variable deconstruction
    let (name, latitude, longitude) = location;
    println!(
        "Location name: {}, latitude: {}, longitude: {}",
        name, latitude, longitude
    );

    let person_name_slice: &str = "Donald Mallard";
    let person_name_string: String = String::from("Donald Mallard");
    // reference to heap memory
    let person_name_slice2: &String = &&person_name_string;
    let person_name_slice3: &str = person_name_string.as_str();

    // concatination
    let duck = "Duck";
    let airlines = "Airlines";

    let airline_name = [duck, " ", airlines].concat();
    println!("{}", airline_name);

    let airline_name2 = format!("{} {}", duck, airlines);
    println!("{}", airline_name2);

    // empty string
    let mut slogan = String::new();

    // append string
    slogan.push_str("We hit the ground");

    // append single char
    slogan.push(' ');

    // concat
    slogan = slogan + "every time";

    println!("{}", slogan);

    let my_inferred_variable = 0;

    // casting

    let float_thirty_two: f32 = 17.2;
    let unsigned_eight: u8 = 5;

    let cast_unsigned_either = unsigned_eight as f32;

    let result = float_thirty_two / cast_unsigned_either;
    println!("{}", result);

    // immutable by default... use mut to define as mutable

    // scope and shadow example
    let scope = "outer scope";
    println!("{}", scope);
    {
        let scope = "inner scope";
        println!("{}", scope);
    }
    println!("{}", scope);
}
