// rpgmap main source file
//
// Program for making simple RPG maps. This is the Rust language implementation.
use clap::{App, Arg};

use rpgtools::map::{GridMap, Renderer};

/// Test whether an input string can be parsed as an int and return a Result
/// as per clap's argument validation API.
fn test_conv_to_int(val: String) -> Result<(), String> {
    let result = val.parse::<usize>();
    match result {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("The value must be an integer")),
    }
}

fn main() {
    let cli = App::new("RPG map generator")
        .version("1.1")
        .author("Aaron Seilis <aaron.seilis@seilis.ca>")
        .about("A simple map generator for role playing games")
        .arg(
            Arg::with_name("width")
                .short("x")
                .long("width")
                .takes_value(true)
                .default_value("100")
                .value_name("INT")
                .validator(test_conv_to_int)
                .help("The horizontal width of the map"),
        )
        .arg(
            Arg::with_name("height")
                .short("y")
                .long("height")
                .takes_value(true)
                .default_value("100")
                .value_name("INT")
                .validator(test_conv_to_int)
                .help("The vertical height of the map"),
        )
        .arg(
            Arg::with_name("map-style")
                .short("s")
                .long("style")
                .takes_value(true)
                .default_value("halls")
                .possible_values(&["halls", "cave"])
                .help("The style of map to generate"),
        )
        .get_matches();

    // Unpack our arguments
    let style = cli.value_of("map-style").unwrap();
    let width: usize = cli.value_of("width").unwrap().parse().unwrap();
    let height: usize = cli.value_of("height").unwrap().parse().unwrap();

    // Initialize our map
    let mut map = GridMap::new(width, height);

    // Build map based on map type
    match style {
        "halls" => {
            map.generate_dungeon(10);
            map.place_entrance_near((width / 2, height / 2))
                .expect("width/height is outside of map");
        }
        "cave" => {
            map.generate_cave(4, 50);
            map.place_entrance_near((width / 2, height / 2))
                .expect("width/height is outside of map");
        }
        _ => unreachable!(),
    }

    let filename = "example.png";
    let renderer = Renderer::new(&map, 10);
    let result = renderer.draw_to_file("example.png");

    match result {
        Ok(_) => println!("Map generated: {}", filename),
        Err(e) => println!("Error: {}", e),
    }
}
