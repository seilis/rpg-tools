#include <cstdlib>
#include <ctime>
#include <iostream>
#include <queue>
#include "gridmap.h"

#include <cairo.h>
#include <boost/random/mersenne_twister.hpp>
#include <boost/random/uniform_int_distribution.hpp>

using std::cout;
using std::endl;

GridMap::GridMap() {}
GridMap::GridMap(const int xmax, const int ymax) 
{
	GridCell blank_cell;
	mMap.resize(xmax);
	for (int i = 0; i < xmax; i++) {
		mMap[i].resize(ymax);
		for (int j = 0; j < ymax; j++) {
			mMap[i][j] = blank_cell;
		}
	}
}

void GridMap::setCell(const int x, const int y, const GridCell cell)
{
	mMap[x][y] = cell;
}

GridCell* GridMap::getCell(const int x, const int y)
{
	return &(mMap[x][y]);
}

int GridMap::getSizeX() {return static_cast<int>(mMap.size());}
int GridMap::getSizeY()
{
	if (this->getSizeX() > 0) {
		return static_cast<int>(mMap[0].size());
	} else {
		return 0;
	}
}

void GridMap::placeRoomDimensions(const int orig_x,const int orig_y,const int wall_h,const int wall_v)
{
	if (orig_x < 0 || orig_y < 0) {
		std::cerr << "ERROR: room origin coordinates are outside of the map." << std::endl;
		std::cerr << "\t Coordinates: ("<< orig_x << ", " << orig_y << ")." << std::endl;
		return;
	}

	int max_x = this->getSizeX();
	int max_y = this->getSizeY();

	if (	orig_x+wall_h > max_x ||
		orig_x+wall_h < 0 ||
		orig_y+wall_v > max_y ||
		orig_y+wall_v < 0 )
	{
		std::cerr << "ERROR: room boundaries are outside of the map." << std::endl;
		return;
	}

	int lower_x, upper_x;
	int lower_y, upper_y;

	if (wall_h > 0) {
		lower_x = orig_x;
		upper_x = orig_x+wall_h;
	} else {
		lower_x = orig_x+wall_h;
		upper_x = orig_x;
	}

	if (wall_v > 0) {
		lower_y = orig_y;
		upper_y = orig_y+wall_v;
	} else {
		lower_y = orig_y+wall_v;
		upper_y = orig_y;
	}

	for (int i = lower_x; i < upper_x; i++) {
		for (int j = lower_y; j < upper_y; j++) {
			if (this->mMap[i][j].isEntrance() == false) {
				this->mMap[i][j] = GridCell(Area_Type::Room);
			}
		}
	}
}

void GridMap::placeRoom(const int x0, const int y0, const int x1, const int y1)
{
	int lower_x, lower_y, upper_x, upper_y;

	if (x1 > x0) {
		lower_x = x0;
		upper_x = x1;
	} else {
		lower_x = x1;
		upper_x = x0;
	}

	if (y1 > y0) {
		lower_y = y0;
		upper_y = y1;
	} else {
		lower_y = y1;
		upper_y = y0;
	}

	if (lower_x < 0) {
		lower_x = 0;
	}

	if (lower_y < 0) {
		lower_y = 0;
	}

	if (upper_x >= this->getSizeX()) {
		upper_x = this->getSizeX() - 1;
	}

	if (upper_y >= this->getSizeY()) {
		upper_y = this->getSizeY() - 1;
	}

	for (int i = lower_x; i <= upper_x; i++) {
		for (int j = lower_y; j <= upper_y; j++) {
			this->mMap[i][j] = GridCell(Area_Type::Room);
		}
	}
}

void GridMap::placeHallway(const int orig_x, const int orig_y, const int dest_x, const int dest_y, const int width, const Route_t route) 
{
	static int salt = 0;
	Route_t route_type = route;

	if (route == Route_t::Manhattan) {
		salt += 1;
		boost::random::mt19937 gen(std::time(0)+salt);
		boost::random::uniform_int_distribution<> dist(0,1);

		int num = dist(gen);

		cout << num << endl;

		if (num == 0) {
			route_type = Route_t::Horizontal_First;
		} else {
			route_type = Route_t::Vertical_First;
		}
	} else if (route != Route_t::Horizontal_First && route != Route_t::Vertical_First) {
		cout << "WARNING: Only Manhattan routing has been implemented." << endl;
	}

	if (route_type == Route_t::Horizontal_First) {
		this->placeRoom(orig_x,orig_y,dest_x,orig_y);
		this->placeRoom(dest_x,orig_y,dest_x,dest_y);
	} else if (route_type == Route_t::Vertical_First) {
		this->placeRoom(orig_x,orig_y,orig_x,dest_y);
		this->placeRoom(orig_x,dest_y,dest_x,dest_y);
	} else {
		cout << "WARNING: Hallway not placed due to lack of implementation..." << endl;
	}
}

void GridMap::placeRandomRoom(const int scale, bool connected)
{
	// Salt the RNG so that multiple calls generate different numbers.
	static int salt = 0;
	salt += 1;

	// Set up random number generator
	boost::random::mt19937 gen(std::time(0)+salt);
	boost::random::uniform_int_distribution<> dist(2,scale);

	// Generate width and height of the room.
	int width = dist(gen);
	int height = dist(gen);

	// Generate location of room.
	boost::random::uniform_int_distribution<> orig_x(1,this->getSizeX());
	boost::random::uniform_int_distribution<> orig_y(1,this->getSizeY());
	int x0 = orig_x(gen);
	int y0 = orig_y(gen);

	if (connected) {
		int xorig = x0+width/2;
		int yorig = y0+height/2;
		if (xorig > this->getSizeX() || yorig > this->getSizeY()) {
			return;
		}

		int x, y;
		this->findNearestConnected(xorig,yorig,x,y);
		this->placeHallway(xorig,yorig,x,y);
		cout << "Room coords (" << xorig << ", "<<yorig << ")"<< endl;
		cout << "Connect     (" << x << ", " << y << ")" << endl;
	}

	// Place room on the map.
	this->placeRoom(x0-width/2,y0-height/2,x0+width/2,y0+height/2);
}

void GridMap::findNearestConnected(const int x0, const int y0, int& x, int& y)
{
	std::queue< std::vector<int> > rooms;
	int radius = 0;
	bool found = false;

	while (!found) {
		if (found) {
			cout << "How the fuck did I get here..." << endl;
		}

		radius += 1;

		int xmin = x0-radius;
		int xmax = x0+radius;
		int ymin = y0-radius;
		int ymax = y0+radius;

		if (xmin < 0) {
			xmin = 0;
		}
		
		if (ymin < 0) {
			ymin = 0;
		}

		if (xmax >= this->getSizeX()) {
			xmax = this->getSizeX()-1;
		}

		if (ymax >= this->getSizeY()) {
			ymax = this->getSizeY()-1;
		}

		if (	xmin == 0 
			&& ymin == 0 
			&& xmax == (this->getSizeX()-1) 
			&& ymax == (this->getSizeY()-1))
		{
			//TODO Throw exception instead.
			found = true;
			break;
		}

		// Scan bottom X
		for (int i = xmin; i <= xmax; i++) {
			if ((this->getCell(i,ymin))->isRoom()) {
				found = true;
				std::vector<int> coord(2);
				coord[0] = i;
				coord[1] = ymin;
				rooms.push(coord);
				// NOTE: Do not stop, just keep going and add to rooms queue.
			}
		}

		// Scan for top X
		for (int i = xmin; i <= xmax; i++) {
			if ((this->getCell(i,ymax))->isRoom()) {
				found = true;
				std::vector<int> coord(2);
				coord[0] = i;
				coord[1] = ymax;
				rooms.push(coord);
				// NOTE: Do not stop, just keep going and add to rooms queue.
			}
		}

		// Scan for left Y
		for (int i = ymin; i <= ymax; i++) {
			if ((this->getCell(xmin,i))->isRoom()) {
				found = true;
				std::vector<int> coord(2);
				coord[0] = xmin;
				coord[1] = i;
				rooms.push(coord);
				// NOTE: Do not stop, just keep going and add to rooms queue.
			}
		}

		// Scan right left Y
		for (int i = ymin; i <= ymax; i++) {
			if ((this->getCell(xmax,i))->isRoom()) {
				found = true;
				std::vector<int> coord(2);
				coord[0] = xmax;
				coord[1] = i;
				rooms.push(coord);
				// NOTE: Do not stop, just keep going and add to rooms queue.
			}
		}

	}

	if (rooms.size() > 0) {
		static int salt = 0;
		salt += 1;
	
		boost::random::mt19937 gen(std::time(0)+salt);
		boost::random::uniform_int_distribution<> dist(0,rooms.size()-1);
	
		int choice = dist(gen);
	
		for (int i = 0; i < choice; i++) {
			rooms.pop();
		}
	
		// These are the actual coordinates that are returned.
		x = (rooms.front())[0];
		y = (rooms.front())[1];
	} else {
		x = 0;
		y = 0;
	}
}


void GridMap::placeEntrance(const int x, const int y) {
	this->mMap[x][y].setArea(Area_Type::Entrance);
}

void GridMap::generate_random_cells(int limit)
{
	GridCell room_cell(Area_Type::Room);
	// Create the random-number generator and seed with the current time.
	boost::random::mt19937 gen(std::time(0));
	boost::random::uniform_int_distribution<> dist(1,100);
	for (int i = 0; i < this->getSizeX(); i++) {
		for (int j = 0; j < this->getSizeY(); j++) {
			int val = dist(gen);
			if (val >= limit) this->setCell(i,j,room_cell);
		}
	}
}


void GridMap::generate_annealed_random_cells()
{
	GridCell empty_cell(Area_Type::Nothing);
	// Start by generating a random grid.
	this->generate_random_cells(80);
	
	// Anneal by removing stragglers.
	for (int i = 1; i < this->getSizeX()-1; i++) {
		for (int j = 1; j < this->getSizeY()-1; j++) {
			bool alone = mMap[i-1][j].isEmpty() 
				&& mMap[i][j-1].isEmpty() 
				&& mMap[i+1][j].isEmpty() 
				&& mMap[i][j+1].isEmpty();
			if (alone) this->setCell(i,j,empty_cell);
		}
	}
}

void GridMap::draw(cairo_t* ctx) {
	// Use a default of 10px for the map scale.
	this->draw(ctx,10);
}

void GridMap::draw(cairo_t* ctx, const int room_scale)
{
	int max_x = this->getSizeX();
	int max_y = this->getSizeY();

	// TEST
	// Load cobblestone floor
	cairo_surface_t* floor = cairo_image_surface_create_from_png("floor_light.png");

	for (int i = 0; i < max_x; i++) {
		for (int j = 0; j < max_y; j++) {
			GridCell& cell = *(this->getCell(i,j));
			if (cell.isRoom()) {
				// Calculate rectangle coordinates
				int x1 = i*room_scale;
				int y1 = j*room_scale;

				// Draw cell
				cairo_set_source_rgb(ctx,0.8,0.8,0.8);
				cairo_rectangle(ctx,x1,y1,room_scale,room_scale);
				cairo_fill(ctx);
				//
				//TEST cobblestone floor.
			//	cairo_set_source_surface(ctx,floor,x1,y1);
			//	cairo_paint(ctx);
			} else if (cell.isEntrance()) {
				int x1 = i*room_scale;
				int y1 = j*room_scale;

				// Set to bright pink
				cairo_set_source_rgb(ctx,1.0,0.078,0.5764);
				cairo_rectangle(ctx,x1,y1,room_scale,room_scale);
				cairo_fill(ctx);
			} else if (cell.getArea() == Area_Type::Tested) {
				int x1 = i*room_scale;
				int y1 = j*room_scale;

				// Set to bright pink
				cairo_set_source_rgb(ctx,0,0,1.0);
				cairo_rectangle(ctx,x1,y1,room_scale,room_scale);
				cairo_fill(ctx);
			}

			if (cell.getVertWall() == Wall_Type::Wall) {
				int x1 = i*room_scale;
				int y1 = j*room_scale;

				// Draw wall
				//TODO
			}

			if (cell.getHorizWall() == Wall_Type::Wall) {
				int x1 = i*room_scale;
				int y1 = j*room_scale;

				//Draw wall
				//TODO
			}
		}
	}
}

bool GridMap::cave_anneal_cell(const int i, const int j)
{
	int neighbours = 0;

	bool onEdge = this->onEdge(i,j);

	if (onEdge == false) {
		neighbours += (this->getCell(i-1,j-1))->isRoom() ? 1 : 0;
		neighbours += (this->getCell(i  ,j-1))->isRoom() ? 1 : 0;
		neighbours += (this->getCell(i+1,j-1))->isRoom() ? 1 : 0;
		neighbours += (this->getCell(i-1,j  ))->isRoom() ? 1 : 0;
		neighbours += (this->getCell(i+1,j  ))->isRoom() ? 1 : 0;
		neighbours += (this->getCell(i-1,j+1))->isRoom() ? 1 : 0;
		neighbours += (this->getCell(i  ,j+1))->isRoom() ? 1 : 0;
		neighbours += (this->getCell(i+1,j+1))->isRoom() ? 1 : 0;

		// Not technically a neighbour, but simplifies the conditional.
		neighbours += (this->getCell(i,j))->isRoom() ? 1 : 0;

		if (neighbours >= 5) {
			return true;
		} else {
			return false;
		}
	} else {
		return false;
	}
}

void GridMap::generate_cave_iteration()
{
	GridMap tmp_map(this->getSizeX(),this->getSizeY());
	GridCell cave_cell(Area_Type::Room);
	GridCell rock_cell(Area_Type::Nothing);

	for (int i = 0; i < tmp_map.getSizeX(); i++) {
		for (int j = 0; j < tmp_map.getSizeY(); j++) {
			if (cave_anneal_cell(i,j) == true) {
				tmp_map.setCell(i,j,cave_cell);
			} else {
				tmp_map.setCell(i,j,rock_cell);
			}
		}
	}

	(*this) = tmp_map;
}

void GridMap::generate_cave(int num_iterations,int seed_limit)
{
	this->generate_random_cells(seed_limit);
	for (int i = 0; i < num_iterations; i++) {
		this->generate_cave_iteration();
	}
}

bool GridMap::onEdgeX(const int x) {
	if ( x == 0 || x == this->getSizeX()-1) {
		return true;
	} else {
		return false;
	}
}

bool GridMap::onEdgeY(const int y) {
	if ( y == 0 || y == this->getSizeY()-1) {
		return true;
	} else {
		return false;
	}
}

bool GridMap::onEdge(const int x, const int y) {
	return onEdgeX(x) || onEdgeY(y);
}

void GridMap::create_test_map()
{
	// 1x1 room at 1.
	this->placeRoom(1,1,1,1);

	// 4x4 room at 3,1
	this->placeRoom(3,1,4,4);

	// Create a set of hallways:
	this->placeHallway(8,1,11,4,1,Route_t::Horizontal_First);
	this->placeHallway(13,1,16,4,1,Route_t::Vertical_First);
	this->placeHallway(18,4,21,1,1,Route_t::Horizontal_First);
	this->placeHallway(23,4,26,1,1,Route_t::Vertical_First);
	this->placeHallway(31,1,28,4,1,Route_t::Horizontal_First);
	this->placeHallway(36,1,33,4,1,Route_t::Vertical_First);
}
