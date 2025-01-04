//! Program for making simple RPG maps. This is the Rust language implementation.
use clap::{command, value_parser, Arg};
use egui;

use rpgtools::error::Result;
use rpgtools::map::gridmap::AreaType;
use rpgtools::map::{gridmap::Point, GridMap};

fn main() -> Result<()> {
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
        .expect("failed to get style; this is a bug")
        .to_string();
    let width: usize = *cli
        .get_one::<u64>("width")
        .expect("failed to get width; this is a bug") as usize;
    let height: usize = *cli
        .get_one::<u64>("height")
        .expect("failed to get height; this is a bug") as usize;
    let num_rooms: usize = *cli
        .get_one::<u64>("num_rooms")
        .expect("failed to get num_rooms; this is a bug") as usize;

    // Initialize our map
    let mut map = GridMap::new(width, height);

    // Build map based on map type
    match style.as_str() {
        "halls" => {
            map.generate_dungeon(num_rooms, 5);
            let point: Point = (width / 2, height / 2).try_into().unwrap();
            map.place_entrance_near(point)
                .expect("width/height is outside of map");
        }
        "cave" => {
            map.generate_cave(4, 50);
            let point: Point = (width / 2, height / 2).try_into().unwrap();
            map.place_entrance_near(point)
                .expect("width/height is outside of map");
        }
        _ => unreachable!(),
    }

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "RPG Map",
        options,
        Box::new(|_cc| Ok(Box::new(RpgMapGui::new(map)))),
    )?;

    Ok(())
}

enum Tool {
    CellPainter(AreaType),
    // Move
    // CellSelection
    // ???
}

impl Default for Tool {
    fn default() -> Self {
        Self::CellPainter(AreaType::Room)
    }
}

struct RpgMapGui {
    // Map state
    map: GridMap,
    // Current Tool selection
    tool: Tool,
    // Mouse state
    dragging: bool,
}

impl RpgMapGui {
    fn new(map: GridMap) -> Self {
        let tool = Tool::default();
        let dragging = false;
        Self {
            map,
            tool,
            dragging,
        }
    }
}

impl eframe::App for RpgMapGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Save").clicked() {
                        // TODO: SAVE!
                    }
                });
                ui.menu_button("Generate", |ui| {
                    if ui.button("Dungeon").clicked() {
                        // Generate a dungeon!
                        self.map.generate_dungeon(10, 5);
                        self.map
                            .place_entrance_near((0, 0))
                            .expect("failed to place entrance");
                    }

                    if ui.button("Cave").clicked() {
                        // Generate a cave!
                        self.map.generate_cave(4, 50);
                        self.map
                            .place_entrance_near((0, 0))
                            .expect("failed to place entrance");
                    }
                });
            });
        });

        egui::SidePanel::left("edit_widgets").show(ctx, |ui| {
            ui.label("Edit");

            if ui.button("Room").clicked() {
                // Set the tool type to room
                self.tool = Tool::CellPainter(AreaType::Room);
            }

            if ui.button("Nothing").clicked() {
                // Set the tool type to nothing
                self.tool = Tool::CellPainter(AreaType::Nothing);
            }

            if ui.button("Entrance").clicked() {
                // Set the tool type to Entrance
                self.tool = Tool::CellPainter(AreaType::Entrance);
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let cell_size = 20.0;

            let (num_x, num_y) = self.map.get_limits();

            egui::ScrollArea::both().show(ui, |ui| {
                let scroll_offset = ui.cursor().left_top();

                // Figure out if we're clicking in this region
                ctx.input(|input| {
                    if input.pointer.primary_down() {
                        self.dragging = true;
                    }
                    if input.pointer.primary_released() {
                        self.dragging = false;
                    }
                });

                for x in 0..num_x {
                    for y in 0..num_y {
                        let cell_x = scroll_offset.x + x as f32 * cell_size;
                        let cell_y = scroll_offset.y + y as f32 * cell_size;

                        let point: Point = (x, y).try_into().unwrap();

                        let cell = ui.allocate_rect(
                            egui::Rect::from_min_size(
                                egui::pos2(cell_x, cell_y),
                                egui::vec2(cell_size, cell_size),
                            ),
                            egui::Sense::drag(), // "click_and_drag" has latency
                        );

                        // TODO: Refactor this into an into() call.
                        let color = match self.map.get_cell_ref(point).area() {
                            AreaType::Room => egui::Color32::LIGHT_GRAY,
                            AreaType::Entrance => egui::Color32::RED,
                            AreaType::Nothing => egui::Color32::DARK_GRAY,
                        };

                        ui.painter().rect_filled(cell.rect, 0.0, color);

                        // Draw the grid
                        ui.painter().rect_stroke(
                            cell.rect,
                            0.0,
                            egui::Stroke::new(1.0, egui::Color32::BLACK),
                        );

                        // If the mouse main button is down then we may need to
                        // set a cell.
                        if self.dragging
                            && cell
                                .rect
                                .contains(ctx.pointer_hover_pos().unwrap_or_default())
                        {
                            if let Tool::CellPainter(ref area) = &self.tool {
                                self.map.get_cell_mut(point).set_area(area.to_owned());
                            }
                        } else if ui
                            .interact(cell.rect, egui::Id::new(point), egui::Sense::click())
                            .clicked()
                        {
                            if let Tool::CellPainter(ref area) = &self.tool {
                                self.map.get_cell_mut(point).set_area(area.to_owned());
                            }
                        }
                    }
                }
            });
        });
    }
}
