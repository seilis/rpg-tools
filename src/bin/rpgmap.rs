// rpgmap main source file
//
// Program for making simple RPG maps. This is the Rust language implementation.
use clap::{App, Arg};

use rpgtools::map::{GridMap, Renderer};

/// Test whether an input string can be parsed as an int and return a Result
/// as per clap's argument validation API.
fn test_conv_to_int(val: &str) -> Result<(), String> {
    let result = val.parse::<usize>();
    match result {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("The value must be an integer")),
    }
}

fn main() {
    let cli = App::new("RPG map generator")
        .version("1.2.0")
        .author("Aaron Seilis <aaron.seilis@seilis.ca>")
        .about("A simple map generator for role playing games")
        .arg(
            Arg::with_name("width")
                .short('x')
                .long("width")
                .takes_value(true)
                .default_value("50")
                .value_name("INT")
                .validator(test_conv_to_int)
                .help("The horizontal width of the map"),
        )
        .arg(
            Arg::with_name("height")
                .short('y')
                .long("height")
                .takes_value(true)
                .default_value("50")
                .value_name("INT")
                .validator(test_conv_to_int)
                .help("The vertical height of the map"),
        )
        .arg(
            Arg::with_name("map-style")
                .short('s')
                .long("style")
                .takes_value(true)
                .default_value("halls")
                .possible_values(&["halls", "cave"])
                .help("The style of map to generate"),
        )
        .arg(
            Arg::with_name("scale")
                .short('S')
                .long("scale")
                .takes_value(true)
                .default_value("25")
                .value_name("INT")
                .validator(test_conv_to_int)
                .help("The number of pixels for each square"),
        )
        .arg(
            Arg::with_name("output")
                .short('o')
                .long("output")
                .takes_value(true)
                .default_value("rpgmap.png")
                .value_name("NAME")
                .help("The name of the output file"),
        )
        .arg(
            Arg::with_name("num_rooms")
                .long("num-rooms")
                .takes_value(true)
                .default_value("30")
                .value_name("INT")
                .validator(test_conv_to_int)
                .help("The number of rooms to generate")
        )
        .arg(
            Arg::with_name("room_size")
                .long("room-size")
                .takes_value(true)
                .default_value("10")
                .value_name("INT")
                .validator(test_conv_to_int)
                .help("The size of generated rooms")
        )
        .get_matches();

    // Unpack our arguments
    let style = cli.value_of("map-style").unwrap();
    let width: usize = cli.value_of("width").unwrap().parse().unwrap();
    let height: usize = cli.value_of("height").unwrap().parse().unwrap();
    let scale: usize = cli.value_of("scale").unwrap().parse().unwrap();
    let filename: String = cli.value_of("output").unwrap().parse().unwrap();
    let num_rooms: i64 = cli.value_of("num_rooms").unwrap().parse().unwrap();

    // Initialize our map
    let mut map = GridMap::new(width, height);

    // Build map based on map type
    match style {
        "halls" => {
            map.generate_dungeon(num_rooms, 5);
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

    let renderer = Renderer::new(&map, scale);
    let result = renderer.draw_to_file(&filename);

    match result {
        Ok(_) => println!("Map generated: {}", filename),
        Err(e) => println!("Error: {}", e),
    }
}
