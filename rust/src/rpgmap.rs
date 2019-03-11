// rpgmap main source file
//
// Program for making simple RPG maps. This is the Rust language implementation.
//use std::clone::Clone;

extern crate rpgtools;
use rpgtools::GridMap;



fn main() {
    let mut map = GridMap::new(100, 100);
    //map.generate_cave(4, 51);
    //map.generate_random_cells(80);


    //// Top left
    //map.place_room((0, 0), (0, 0));
    //map.place_room((1, 1), (3, 3));

    //// Bottom left
    //map.place_room((0, 99), (0, 99));
    //map.place_room((1, 98), (3, 96));

    //// Bottom
    //map.place_room((99, 0), (99, 0));
    //map.place_room((99, 99), (99, 99));

    //map.place_room((47, 47), (53, 53));
    map.place_entrance((50, 50));

    for _ in 0..30 {
        map.place_random_room(10, true);
    }


    let filename = "example.png";
    let result = map.draw_to_file("example.png", 10);

    match result {
        Ok(_) => println!("Dungeon generated: {}", filename),
        Err(e) => println!("Error: {}", e),
    }
}
