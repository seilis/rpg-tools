// rpgmap main source file
//
// Program for making simple RPG maps. This is the Rust language implementation.

extern crate rpgtools;
use rpgtools::GridMap;

extern crate clap;
use clap::{Arg, App};

/// Test whether an input string can be parsed as an int and return a Result
/// as per clap's argument validation API.
fn test_conv_to_int(val: String) -> Result<(), String> {
    let result = val.parse::<usize>();
    match result {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("The value must be an integer"))
    }
}

fn main() {
    let cli = App::new("RPG map generator")
                      .version("0.1")
                      .author("Aaron Seilis <aaron.seilis@seilis.ca>")
                      .about("A simple map generator for role playing games")
                      .arg(Arg::with_name("width")
                           .short("x")
                           .long("width")
                           .takes_value(true)
                           .default_value("100")
                           .value_name("INT")
                           .validator(test_conv_to_int)
                           .help("The horizontal width of the map"))
                      .arg(Arg::with_name("height")
                           .short("y")
                           .long("height")
                           .takes_value(true)
                           .default_value("100")
                           .value_name("INT")
                           .validator(test_conv_to_int)
                           .help("The vertical height of the map"))
                      .arg(Arg::with_name("map-style")
                           .short("s")
                           .long("style")
                           .takes_value(true)
                           .default_value("halls")
                           .possible_values(&["halls", "cave"])
                           .help("The style of map to generate"))
                      .get_matches();

    let style = cli.value_of("map-style").unwrap();
    let width: usize = cli.value_of("width").unwrap().parse().unwrap();
    let height: usize = cli.value_of("height").unwrap().parse().unwrap();

    let mut map = GridMap::new(width, height);

    if style == "halls" {

        map.place_room((width/2-1, height/2-1), (width/2+1, height/2+1));
        map.place_entrance((width/2, height/2));

        for _ in 0..30 {
            map.place_random_room(10, true);
        }
    } else if style == "cave" {
        map.generate_cave(4, 50);
    }


    let filename = "example.png";
    let result = map.draw_to_file("example.png", 10);

    match result {
        Ok(_) => println!("Map generated: {}", filename),
        Err(e) => println!("Error: {}", e),
    }
}
