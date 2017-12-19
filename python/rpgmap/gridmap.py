import collections
import copy
import random

from .route import Route_t


class Area_Type:
    Nothing = 0
    Entrance = 1
    Room = 2
    Stairs = 3
    Tested = 4

class Wall_Type:
    Nothing = 0
    Wall = 1
    Door = 2
    Secret_Door = 3

class Point_Type:
    Nothing = 0
    Pillar = 1

class Accessable_t:
    Yes = True
    No = False
    Unknown = None


class GridCell:
    def __init__(self, area=Area_Type.Nothing, horizontal_wall=Wall_Type.Nothing, 
                       vertical_wall=Wall_Type.Nothing, point=Point_Type.Nothing, 
                       access=None):
        self.accessable = access
        self.areaType = area
        self.verticalWallType = vertical_wall
        self.horizontalWallType = horizontal_wall
        self.pointType = point

    @property
    def isAccessable(self):
        return bool(self.accessable)

    @property
    def isEmpty(self):
        return (self.areaType == Area_Type.Nothing and
                self.verticalWallType == Wall_Type.Nothing and
                self.horizontalWallTYpe == Wall_Type.Nothing and
                self.pointType == Point_Type.Nothing)

    @property
    def isEntrance(self):
        return self.areaType == Area_Type.Entrance

    @property
    def isRoom(self):
        return self.areaType == Area_Type.Room

    def setArea(self, areaType):
        self.areaType = areaType





class GridMap:
    def __init__(self, xmax=None, ymax=None):
        self._xmax = xmax
        self._ymax = ymax

        if self._xmax and self._ymax:
            self._cells = [[GridCell() for y in range(self._ymax)] for x in range(self._xmax)]


    def setCell(self, x, y, cell):
        self._cells[x][y] = cell

    def getCell(self, x, y):
        return self._cells[x][y]

    @property
    def sizeX(self):
        return self._xmax

    @property
    def sizeY(self):
        return self._ymax

    def placeRoom(self, x0, y0, x1, y1):
        # Ensure that the order is well-formed 
        lower_x, upper_x = sorted((x0, x1))
        lower_y, upper_y = sorted((y0, y1))

        # Clamp the lower bounds to 0
        lower_x = max(0, lower_x)
        lower_y = max(0, lower_y)
            
        # Clamp the upper bounds to max_x, max_y
        upper_x = min(self.sizeX, upper_x)
        upper_y = min(self.sizeY, upper_y)

        for i in range(lower_x, upper_x):
            for j in range(lower_y, upper_y):
                self._map[i][j].areaType = Area_Type.Room

    def placeRoomDimensions(self, orig_x, orig_y, wall_h, wall_v):
        if orig_x < 0 or orig_y < 0:
            print("ERROR: room origin coordinates are outside the map\n"
                  f"\tCoordinates: ({orig_x}, {orig_y}).", file=sys.stderr)
            return
        
        max_x = self.sizeX
        max_y = self.sizeY

        if not (0 < orig_x+wall_h < max_x) or not (0 < orig_y+wall_v < max_y):
            print("ERROR: room boundaries are outside the map", file=sys.stderr)
            return

        if wall_h > 0:
            lower_x = orig_x
            upper_x = orig_x+wall_h
        else:
            lower_x = orig_x+wall_h
            upper_x = orig_x

        if wall_v > 0:
            lower_y = orig_y
            upper_y = orig_y+wall_v
        else:
            lower_y = orig_y+wall_v
            upper_y = orig_y

        for i in range(lower_x, upper_x):
            for j in range(lower_y, upper_y):
                if not self._map[i][j].isEntrace:
                    self._map[i][j] = GridCell(Area_Type.room)

    def placeHallway(self, orig_x, orig_y, dest_x, dest_y, width=1, route=Route_t.Manhattan):
        route_type = route

        if route == Route_t.Manhattan:
            num = random.randint(0, 1)
            print(num)
            if num == 0:
                route_type = Route_t.Horizontal
            else:
                route_type = Route_t.Vertical
        elif route not in {Route_t.Horizontal, Route_t.Vertical}:
            print("WARNING: Only Manhattan routing has been implemented.", file=sys.stderr)

        if route_type == Route_t.Horizontal:
            self.placeRoom(orig_x, orig_y, dest_x, orig_y)
            self.placeRoom(dest_x, orig_y, dest_x, dest_y)
        else:
            self.placeRoom(orig_x, orig_y, orig_x, dest_y)
            self.placeRoom(orig_x, dest_y, dest_x, dest_y)


    def placeRandomRoom(self, scale, connected=True):
        width = random.randint(2, scale)
        height = random.randint(2, scale)

        x0 = random.randint(1, self.sizeX)
        x1 = random.randint(1, self.sizeY)

        if connected:
            xorig = x0+width//2
            yorig = y0+height//2
            if xorig > self.sizeX or yorig > self.sizeY:
                return
            (x, y) = self.findNearestConnected(xorig, yorig)
            self.placeHallway(xorig, yorig, x, y)
            print(f"Room coords ({xorig}, {yorig})\n"
                  f"Connect     ({x}, {y})")

            self.placeRoom(x0-width//2, y0-height//2, x0+width//2, y0+height//2)


    def placeEntrance(self, x, y):
        # Do in a for loop so that we can keep looking if the first one isn't a room
        for xShift in range(x, self.sizeX):
            if self._cells[xShift][y].isRoom:
                self._cells[xShift][y].setArea(Area_Type.Entrance)
                break

    def findNearestConnected(self, x0, y0):
        rooms = collections.deque()
        radius = 0
        found = False

        while not found:
            radius += 1
            xmin = x0-radius
            xmax = x0+radius
            ymin = y0-radius
            ymax = y0+radius

            if xmin < 0:
                xmin = 0
            if ymin < 0:
                ymin = 0

            if xmax >= self.sizeX:
                xmax = self.sizeX - 1
            if ymax >= self.sizeY:
                ymax = self.sizeY - 1

            if xmin == 0 and ymin == 0 and xmax == self.sizeX-1 and ymax == self.sizeY-1:
                # Todo: exception or other?
                found = True

            # Scan bottom x
            for i in range(xmin, xmax+1):
                if self.getCell(i, ymax).isRoom:
                    found = True
                    coord = (i, ymax)
                    rooms.append(coord)
                    # NOTE: Do not stop, just keep going and add to rooms queue

            # Scan top x
            for i in range(xmin, xmax+1):
                if self.getCell(i, ymin).isRoom:
                    found = True
                    coord = (i, ymin)
                    rooms.append(coord)
                    # NOTE: Do not stop, just keep going and add to rooms queue

            # Scan for left Y
            for i in range(ymin, ymax+1):
                if self.getCell(xmin, i).isRoom:
                    found = True
                    coord = (xmin, i)
                    rooms.append(coord)
                    # NOTE: Do not stop, just keep going and add to rooms queue

            # Scan for right Y
            for i in range(ymin, ymax+1):
                if self.getCell(xmax, i).isRoom:
                    found = True
                    coord = (xmax, i)
                    rooms.append(coord)
                    # NOTE: Do not stop, just keep going and add to rooms queue

        if rooms:
            choice = random.randint(0, len(rooms)-1)
            return rooms[choice]
        return (0, 0)


    # Limit is a value between 1 and 100. This limit sets the change that the cells are 
    # a room. (Higher means fewer rooms).
    def generate_random_cells(self, limit):
        for i in range(self.sizeX):
            for j in range(self.sizeY):
                val = random.randint(1, 100)
                if val > limit:
                    self.setCell(i, j, GridCell(Area_Type.Room))

    def generate_annealed_random_cells(self):
        # Start by generating a random grid
        self.generate_random_cells(80)

        # Anneal by removing stragglers
        for i in range(1, self.sizeX-1):
            for j in range(1, self.sizeY-1):
                alone = self._map[i-1][j].isEmpty and self._map[i][j-1].isEmpty and self._map[i+1][j].isEmpty and self._map[i][j+1].isEmpty
                if alone:
                    self.setCell(i, j, empty_cell)


    def generate_cave(self, num_iterations, seed_limit=55):
        self.generate_random_cells(seed_limit)
        for i in range(num_iterations):
            self.generate_cave_iteration()
        self.removeOrphans()


    def cave_anneal_cell(self, i, j):
        if not self.onEdge(i, j):
            # Count the number of neighbours
            neighbours = 0
            neighbours += 1 if self.getCell(i-1, j-1).isRoom else 0
            neighbours += 1 if self.getCell(i  , j-1).isRoom else 0
            neighbours += 1 if self.getCell(i+1, j-1).isRoom else 0
            neighbours += 1 if self.getCell(i-1, j  ).isRoom else 0
            neighbours += 1 if self.getCell(i+1, j  ).isRoom else 0
            neighbours += 1 if self.getCell(i-1, j+1).isRoom else 0
            neighbours += 1 if self.getCell(i  , j+1).isRoom else 0
            neighbours += 1 if self.getCell(i+1, j+1).isRoom else 0

            # Technically not a neighbours, but simplifies the conditional.
            neighbours += 1 if self.getCell(i,j).isRoom else 0
            return neighbours >= 5
        return False


    def generate_cave_iteration(self):
        tmp_map = copy.deepcopy(self._cells)

        for i in range(self.sizeX):
            for j in range(self.sizeY):
                if self.cave_anneal_cell(i, j):
                    tmp_map[i][j].areaType = Area_Type.Room
                else:
                    tmp_map[i][j].areaType = Area_Type.Nothing

        self._cells = tmp_map
        return self._cells

    def removeOrphans(self):
        """Remove orphaned rooms from the map"""
        for x in range(self.sizeX):
            for y in range(self.sizeY):
                size = self.getRoomSize(x, y)
                if 0 < size < 15:
                    self.clearRoom(x, y)

    def getRoomSize(self, x, y):
        """Get the size of the room (number of cells) connected to (x,y)"""

        # Output value
        size = 0

        # Process all of the points in a list. Doing this in an iterative fashion because of
        # max recursion limits
        processingQueue = collections.deque([(x,y)])

        # Mask to make sure that we don't revisit rooms
        visited = [[False for y in range(self.sizeY)] for x in range(self.sizeX)]

        # Keep going while we have rooms to process
        while processingQueue:
            # Remove a room from the queue, unpack the values
            currentCell = processingQueue.pop()
            xcur, ycur = currentCell

            # If we have visited this cell before or it is not a "Room" then we should
            # just stop processing and move on to the next.
            if visited[xcur][ycur] or self._cells[xcur][ycur].areaType != Area_Type.Room:
                continue

            # We got here so it's a room that we haven't visited. Mark it as visited now
            visited[xcur][ycur] = True

            # This is a room and we haven't visited here before so add one to size
            size += 1

            # Add all neighbours to the queue
            processingQueue.extend([(xcur-1, ycur),
                                    (xcur+1, ycur),
                                    (xcur, ycur-1),
                                    (xcur, ycur+1)])
        return size
        

    def clearRoom(self, x, y):
        if self._cells[x][y].areaType == Area_Type.Nothing:
            return

        processingQueue = collections.deque([(x, y)])

        while processingQueue:
            xcur, ycur = processingQueue.pop()
            if self._cells[xcur][ycur].areaType != Area_Type.Nothing:
                self._cells[xcur][ycur].areaType = Area_Type.Nothing
                processingQueue.extend([(xcur+1, ycur),
                                        (xcur-1, ycur),
                                        (xcur, ycur+1),
                                        (xcur, ycur-1)])
        return




    def draw(self, ctx, scale=10):
        max_x = self.sizeX
        max_y = self.sizeY

        for i in range(max_x):
            for j in range(max_y):
                cell = self.getCell(i, j)

                if cell.isRoom:
                    x1 = i*scale
                    y1 = j*scale

                    ctx.set_source_rgb(0.8, 0.8, 0.8)
                    ctx.rectangle(x1, y1, scale, scale)
                    ctx.fill()
                elif cell.isEntrance:
                    x1 = i*scale
                    y1 = j*scale

                    ctx.set_source_rgb(1.0, 0.078, 0.5764)
                    ctx.rectangle(x1, y1, scale, scale)
                    ctx.fill()
                elif cell.areaType == Area_Type.Tested:
                    x1 = i*scale
                    y1 = i*scale

                    ctx.set_source_rgb(0, 0, 1.0)
                    ctx.rectangle(x1, y1, scale, scale)
                    ctx.fill()

    def onEdge(self, x, y):
        return x == 0 or x == self.sizeX-1 or y == 0 or y == self.sizeY-1
