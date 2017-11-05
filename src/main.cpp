#include <stdio.h>
#include <string>
#include <cairo.h>
#include <iostream>

// May not be correct include.
#include <boost/program_options.hpp>

#include "gridcell.h"
#include "gridmap.h"

// Program options namespace.
namespace po = boost::program_options;
using std::string;


/*TODO: This function is useful and will eventually be included
 * correctly. For now, it is just an experimental sandbox.
 * draw_compass
 *
 * This function draws a map compass at the specified location on the map.
 */
void draw_compass(cairo_t* ctx, int x, int y, double scale, double rotation)
{
	// Use a default of 100 x 100 px when scale = 1.
	// Draw horizontal line first	
	cairo_move_to(ctx, x-25*scale, scale*50+y);
	cairo_line_to(ctx, x+25*scale, scale*50+y);
	cairo_stroke(ctx);

	cairo_rectangle(ctx, 100, 100, 500, 500);
	cairo_set_line_width(ctx, 14);
	cairo_set_line_join(ctx, CAIRO_LINE_JOIN_BEVEL);
	cairo_stroke(ctx);

	cairo_set_line_width(ctx, 3);
	cairo_set_source_rgb(ctx, 1, 1, 1);
	cairo_rectangle(ctx, 200, 200, 600, 600);

	cairo_fill(ctx);
}

/*
 * draw_square_grid
 *
 * This function draws a square grid with a square extent. It will draw
 * one external set of lines to close off the grid.
 */
void draw_square_grid(cairo_t* ctx, int max, int sep)
{
	// Calculate the maximum index (integer floor intended).
	int max_ind = max / sep;

	cairo_set_line_width(ctx, 1);
	for (int i = 0; i < max_ind + 1; i++) {
		// Draw horizontal lines
		cairo_move_to(ctx, 0, static_cast<float>(i*sep)+0.5);
		cairo_line_to(ctx, max, static_cast<float>(i*sep)+0.5);
		cairo_stroke(ctx);

		// Draw vertical lines
		cairo_move_to(ctx, static_cast<float>(i*sep)+0.5, 0);
		cairo_line_to(ctx, static_cast<float>(i*sep)+0.5, max);
		cairo_stroke(ctx);
	}
}

/*
 * draw_solid_background
 *
 * This function uses the current RGB set to draw a rectangle of a solid
 * color across the entire image. This will wipe out any data on the image
 * up to this point, so only use it at the beginning. It makes it so that
 * the canvas has a solid color starting point instead of a transparent
 * background.
 */
void draw_solid_background(cairo_t* ctx, int x_max, int y_max)
{
	cairo_rectangle(ctx, 0, 0, x_max, y_max);
	cairo_fill(ctx);
}

/*
 * draw_map
 *
 * Take voxel-style information of a map and draw it using cairo.
 *
 * cell_px is the linear number of pixels that make up a cell.
 */

/*
void draw_map(cairo_t* ctx, GridMap map, int cell_px)
{
	int max_x = map.getSizeX();
	int max_y = map.getSizeY();

	for (int i = 0; i < max_x; i++) {
		for (int j = 0; j < max_y; j++) {
			GridCell& cell = *map.getCell(i,j);
			if (cell.isRoom()) {
				// Calculate rectangle coordinates
				int x1 = i*cell_px;
				int y1 = j*cell_px;

				// Draw cell
				cairo_rectangle(ctx,x1,y1,cell_px,cell_px);
				cairo_fill(ctx);
			}
		}
	}
}
*/

int main(int argc, char* argv[]) 
{
	// Options support
	po::options_description desc("Allowed options");

	po::positional_options_description pos_opts;
	pos_opts.add("output file", -1);

	// Add options defines an object that calling () multiple times will
	// add options. Kind of cool, but definitely non-standard.
	desc.add_options()
		("help,h", "Prints the help message")
		("version", "prints the program version")
		("verbose,v", "prints extra information about the map")
		("map-type,m", po::value<string>(), "Type of map")
		("size-x,x", po::value<int>()->default_value(50), "size of map on X-axis")
		("size-y,y", po::value<int>()->default_value(50), "size of map on Y-axis")
		("output,o", po::value<string>()->default_value("map.png"), "name of the output file")
		;

	// Create a variable map.
	po::variables_map var_map;

	// Parse the command line.
	try {
		po::store(po::command_line_parser(argc, argv).options(desc).positional(pos_opts).run(), var_map);
	
		// Not sure what this does yet.
		po::notify(var_map);
	
		/**************************
		 * Print help message on "help"
		 *************************/
		if (var_map.count("help")) {
			std::cout << desc << std::endl;
			return 0;
		}
	} catch (po::error& e) {
		std::cerr << "ERROR: " << e.what() << std::endl << std::endl;
		std::cerr << desc << std::endl;
		return 1;
	}

	/****************************************************
	 * Map logic starts here.
	 ***************************************************/
	// Set up map dimensions.
	
	// Default is 50x50.
	int x = 50;
	int y = x;

	if (var_map.count("size-x")) {
		x = var_map["size-x"].as<int>();
	}

	if (var_map.count("size-y")) {
		y = var_map["size-y"].as<int>();
	}

	// Create a blank map.
	GridMap map(x,y);

	map.generate_cave(4, 50);
	// Generate an annealed random map
//	map.generate_annealed_random_cells();
/*	for (int i = 0; i < 2050; i++) {
		map.placeRandomRoom(10);
	}*/

	
//	map.placeRoom(1,3,5,4);

//	map.placeRoom(40,40,60,60);
//	map.placeRoom(80,90,110,1);
//	map.placeRoom(80,90,1,10);
//	map.placeRoom(80,100,5,1);
//
/*
	map.placeRoom(47,47,50,50);
	map.placeEntrance(50,50);
*/
	/*
	map.placeEntrance(100,100);
	map.placeEntrance(102,100);
	map.placeEntrance(102,102);
	map.placeEntrance(100,98);
	map.placeEntrance(102,98);
	map.placeEntrance(100,102);
	map.placeEntrance(98,102);
	map.placeEntrance(98,100);
	map.placeEntrance(98,98);
	*/

//	map.placeHallway(5,5,10,10,1,Route_t::Horizontal_First);
//	map.placeHallway(20,10,15,5,1,Route_t::Vertical_First);


//	map.placeHallway(100,100,200,250);
//	map.placeHallway(300,300,325,250);
	
/*
	
	for (int i = 0; i < 30; i++) {
		std::cout << "Placing room (" << i << "/4000)."<< std::endl;
		map.placeRandomRoom(5);
	}

	for (int i = 0; i < 2; i++) {
		map.placeRandomRoom(10);
	}

*/
/*
	for (int i = 0; i < 300; i++) {
		map.placeRandomRoom(20);
	}*/
//	map.placeHallway(358,32,109,108,1,Route_t::Horizontal_First);
	

//	map.create_test_map();

	// Load cobblestone image (10 px x 10 px)

	// Set up Cairo drawing.
	cairo_surface_t* surface = cairo_image_surface_create(CAIRO_FORMAT_ARGB32,x*70,y*70);
	cairo_t* cr = cairo_create(surface);

	// Set background to black
	cairo_set_source_rgb(cr,0.2,0.2,0.2);
 	draw_solid_background(cr, x*70, y*70);

	// Set rooms to grey, draw map
//	cairo_set_source_rgb(cr,0.8,0.8,0.8);
//	draw_map(cr,map,10);
	map.draw(cr,70);
	// Draw cell grid
//	cairo_set_source_rgba(cr,0.3,0.3,0.3,0.5);
//	draw_square_grid(cr,x*10,10);

	// Clean up and write to file
	cairo_destroy(cr);
	string output_file = var_map["output"].as<string>();
	cairo_surface_write_to_png(surface, output_file.c_str());
	cairo_surface_destroy(surface);
}	
