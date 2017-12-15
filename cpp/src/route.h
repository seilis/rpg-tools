#ifndef RPGMAP_ROUTE_H
#define RPGMAP_ROUTE_H

enum class Route_t {
	Horizontal_First,
	Vertical_First,
	Diagonal,
	Circular,
	Manhattan, // One of horizontal_first or vertical first
	Random
};

#endif // ifndef RPGMAP_ROUTE_H

