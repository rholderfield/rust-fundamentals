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

    // immutable by default... use mut to define as mutable
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

    // scope and shadow example
    let scope = "outer scope";
    println!("{}", scope);
    {
        let scope = "inner scope";
        println!("{}", scope);
    }
    println!("{}", scope);

    // operators

    let modulus = 18 % 7;

    let squared = i32::pow(8, 2);
    println!("{}", squared);

    // raise to an floating point to integer power
    let float_integer = f32::powi(6.5, 3);
    println!("{}", float_integer);

    // raise float to float power
    let float_float = f32::powf(6.5, 3.14);
    println!("{}", float_float);

    // logic op examples
    let have_boarding_pass = true;
    let have_id = false;
    let can_board = have_boarding_pass && have_id;
    println!("{}", can_board);
    {
        let have_id = true;
        let can_board_now = have_boarding_pass && have_id;
        println!("{}", can_board_now);
    }

    {
        const EARTH_RADIUS_IN_KILOMETERS: f64 = 6371.0;

        let kcle_latitude_degrees: f64 = 41.4075;
        let kcle_longitude_degrees: f64 = -81.851111;

        let kslc_latitude_degrees: f64 = 40.7861;
        let kslc_longitude_degrees: f64 = -111.9822;

        let kcle_latitude_radians: f64 = kcle_latitude_degrees.to_radians();
        let kslc_latitude_radians: f64 = kslc_latitude_degrees.to_radians();

        let delta_latitude: f64 = (kcle_latitude_degrees - kslc_latitude_degrees).to_radians();
        let delta_longitude: f64 = (kcle_longitude_degrees - kslc_longitude_degrees).to_radians();

        let inner_central_angle = f64::powi((delta_latitude / 2.0).sin(), 2)
            + kcle_latitude_radians.cos()
                * kslc_latitude_radians.cos()
                * f64::powi((delta_longitude / 2.0).sin(), 2);

        let central_angle: f64 = 2.0 * inner_central_angle.sqrt().asin();

        let distance: f64 = EARTH_RADIUS_IN_KILOMETERS * central_angle;

        println!("The distance between points is {:.1} kilometers", distance);
    }
}
