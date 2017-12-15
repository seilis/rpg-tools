#ifndef RPGMAP_GRIDCELL_H
#define RPGMAP_GRIDCELL_H
enum class Area_Type {
	Nothing,
	Entrance,
	Room,
	Stairs,
	Tested
};

enum class Wall_Type {
	Nothing,
	Wall,
	Door,
	Secret_Door
};

enum class Point_Type {
	Nothing,
	Pillar
};

enum class Accessable_t {
	Yes,
	No,
	Unknown
};

class GridCell {
	public:
		GridCell();
		GridCell(const Area_Type area);
		GridCell(const Area_Type area, const Wall_Type horizontal_wall, const Wall_Type vert_wall);
		GridCell(const Area_Type area, const Wall_Type horizontal_wall, const Wall_Type vert_wall, const Point_Type point);

		void setAccessable(Accessable_t acc);
		void setArea(const Area_Type area);
		void setVertWall(const Wall_Type wall);
		void setHorizWall(const Wall_Type wall);
		void setPoint(const Point_Type point);


		bool isAccessable();
		bool isEmpty();
		bool isEntrance();
		bool isRoom();

		Area_Type getArea();
		Wall_Type getVertWall();
		Wall_Type getHorizWall();
		Point_Type getPoint();
	
	protected:
		Accessable_t mAccessable;
		Area_Type mAreaType;
		Wall_Type mVerticalWallType;
		Wall_Type mHorizontalWallType;
		Point_Type mPointType;
};

#endif // define RPGMAP_GRIDCELL
