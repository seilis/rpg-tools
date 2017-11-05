#ifndef RPGMAP_GRIDMAP
#define RPGMAP_GRIDMAP

#include <cairo.h>
#include <vector>

#include "gridcell.h"
#include "route.h"

class GridMap {
	public:
		GridMap();
		GridMap(const int xmax, const int ymax); // create map with extents.

		void setCell(const int x, const int y, const GridCell cell);
		GridCell* getCell(const int x, const int y);

		int getSizeX();
		int getSizeY();

		void placeRoom(const int x0, const int y0, const int x1, const int y1);
		void placeRoomDimensions(const int orig_x, const int orig_y, const int wall_h, const int wall_v);
		void placeHallway(const int orig_x, const int orig_y, const int dest_x, const int dest_y, const int width = 1, const Route_t route = Route_t::Manhattan);
		void placeRandomRoom(const int scale, bool connected = true);
		void placeEntrance(const int x, const int y);

		void findNearestConnected(const int x0, const int y0, int& x, int& y);

		// Limit is a value between 1 and 100. This limit sets the chance that the cells are 
		// a room. (Higher means fewer rooms).
		void generate_random_cells(int limit);
		void generate_annealed_random_cells();
		void generate_cave(int num_iterations,int seed_limit=55);

		bool cave_anneal_cell(const int i, const int j);
		void generate_cave_iteration();

		void draw(cairo_t* ctx);
		void draw(cairo_t* ctx, const int room_scale);

		void create_test_map();

		bool onEdge(const int x, const int y);
		bool onEdgeX(const int x);
		bool onEdgeY(const int y);

	protected:
		// The map is stored as a vector of integers. The ints determine the block type.
		std::vector< std::vector<GridCell> > mMap;
};

#endif // define RPGMAP_GRIDMAP
