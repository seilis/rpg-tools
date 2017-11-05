#include "gridcell.h"

GridCell::GridCell() {
	this->mAreaType = Area_Type::Nothing;
	this->mVerticalWallType = Wall_Type::Nothing;
	this->mHorizontalWallType = Wall_Type::Nothing;
	this->mPointType = Point_Type::Nothing;
}

GridCell::GridCell(const Area_Type area) {
	this->mAreaType = area;
	this->mVerticalWallType = Wall_Type::Nothing;
	this->mHorizontalWallType = Wall_Type::Nothing;
	this->mPointType = Point_Type::Nothing;
}

GridCell::GridCell(const Area_Type area, const Wall_Type horizontal_wall, const Wall_Type vert_wall) {
	this->mAreaType = area;
	this->mVerticalWallType = vert_wall;
	this->mHorizontalWallType = horizontal_wall;
	this->mPointType = Point_Type::Nothing;
}

GridCell::GridCell(const Area_Type area, const Wall_Type horizontal_wall, const Wall_Type vert_wall, const Point_Type point) {
	this->mAreaType = area;
	this->mVerticalWallType = vert_wall;
	this->mHorizontalWallType = horizontal_wall;
	this->mPointType = point;
}

void GridCell::setAccessable(Accessable_t acc)
{
	this->mAccessable = acc;
}

void GridCell::setArea(const Area_Type area) {
	this->mAreaType = area;
}

void GridCell::setVertWall(const Wall_Type wall) {
	this->mVerticalWallType = wall;
}

void GridCell::setHorizWall(const Wall_Type wall) {
	this->mHorizontalWallType = wall;
}

void GridCell::setPoint(const Point_Type point) {
	this->mPointType = point;
}

bool GridCell::isAccessable()
{
	if (this->mAccessable == Accessable_t::Yes) {
		return true;
	} else {
		return false;
	}
}

bool GridCell::isRoom() {
	if (this->mAreaType == Area_Type::Room) {
		return true;
	} else {
		return false;
	}
}

bool GridCell::isEntrance() {
	if (this->mAreaType == Area_Type::Entrance) {
		return true;
	} else {
		return false;
	}
}

bool GridCell::isEmpty() {
	if (this->mAreaType == Area_Type::Nothing && 
			this->mVerticalWallType == Wall_Type::Nothing &&
			this->mHorizontalWallType == Wall_Type::Nothing &&
			this->mPointType == Point_Type::Nothing) {
		return true;
	} else {
		return false;
	}
}

Area_Type GridCell::getArea() {
	return this->mAreaType;
}

Wall_Type GridCell::getVertWall() {
	return this->mVerticalWallType;
}

Wall_Type GridCell::getHorizWall() {
	return this->mHorizontalWallType;
}

Point_Type GridCell::getPoint() {
	return this->mPointType;
}
