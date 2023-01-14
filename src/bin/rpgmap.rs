//! Program for making simple RPG maps. This is the Rust language implementation.
use clap::{command, value_parser, Arg};

use rpgtools::map::{GridMap, Renderer};

fn main() {
    let cli = command!()
        .author("Aaron Seilis <aaron.seilis@seilis.ca>")
        .about("A simple map generator for role playing games")
        .arg(
            Arg::new("width")
                .short('x')
                .long("width")
                .default_value("50")
                .value_name("INT")
                .value_parser(value_parser!(u64).range(1..))
                .help("The horizontal width of the map"),
        )
        .arg(
            Arg::new("height")
                .short('y')
                .long("height")
                .default_value("50")
                .value_name("INT")
                .value_parser(value_parser!(u64).range(1..))
                .help("The vertical height of the map"),
        )
        .arg(
            Arg::new("map-style")
                .short('s')
                .long("style")
                .default_value("halls")
                .value_parser(["halls", "cave"])
                .help("The style of map to generate"),
        )
        .arg(
            Arg::new("scale")
                .short('S')
                .long("scale")
                .default_value("25")
                .value_name("INT")
                .value_parser(value_parser!(u64).range(1..))
                .help("The number of pixels for each square"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .default_value("rpgmap.png")
                .value_name("NAME")
                .help("The name of the output file"),
        )
        .arg(
            Arg::new("num_rooms")
                .long("num-rooms")
                .default_value("30")
                .value_name("INT")
                .value_parser(value_parser!(u64).range(1..))
                .help("The number of rooms to generate"),
        )
        .arg(
            Arg::new("room_size")
                .long("room-size")
                .default_value("10")
                .value_name("INT")
                .value_parser(value_parser!(u64).range(1..))
                .help("The size of generated rooms"),
        )
        .get_matches();

    // Unpack our arguments
    let style: String = cli
        .get_one::<String>("map-style")
        .expect("failed to get style; this is a bug").to_string();
    let width: usize = *cli
        .get_one::<u64>("width")
        .expect("failed to get width; this is a bug") as usize;
    let height: usize = *cli
        .get_one::<u64>("height")
        .expect("failed to get height; this is a bug") as usize;
    let scale: usize = *cli
        .get_one::<u64>("scale")
        .expect("failed to get scale; this is a bug") as usize;
    let filename: String = cli
        .get_one::<String>("output")
        .expect("failed to get filename; this is a bug")
        .to_string();
    let num_rooms: usize = *cli
        .get_one::<u64>("num_rooms")
        .expect("failed to get num_rooms; this is a bug") as usize;

    // Initialize our map
    let mut map = GridMap::new(width, height);

    // Build map based on map type
    match style.as_str() {
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
