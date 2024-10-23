def cell_decomposition(obstacles, start, goal):
    cells = create_cells(obstacles)
    path = find_path_through_cells(cells, start, goal)
    return path

def create_cells(obstacles):
    return [(0, 0, 1, 1), (1, 1, 2, 2)]

def find_path_through_cells(cells, start, goal):
    return [(start), (goal)]

obstacles = [(1, 1, 2, 2)]
path = cell_decomposition(obstacles, (0, 0), (3, 3))
print("Path:", path)