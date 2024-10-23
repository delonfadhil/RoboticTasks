def get_neighbors(node, grid):
    neighbors = []
    directions = [(-1, 0), (1, 0), (0, -1), (0, 1)]
    for d in directions:
        neighbor = (node[0] + d[0], node[1] + d[1])
        if 0 <= neighbor[0] < len(grid) and 0 <= neighbor[1] < len(grid[0]) and grid[neighbor[0]][neighbor[1]] == 0:
            neighbors.append(neighbor)
    return neighbors

def a_star(start, goal, grid):
    open_list = []
    closed_list = set()
    open_list.append(start)

    g = {start: 0}
    f = {start: heuristic(start, goal)}

    came_from = {}

    while open_list:
        current = min(open_list, key=lambda x: f[x])

        if current == goal:
            path = []
            while current in came_from:
                path.append(current)
                current = came_from[current]
            path.append(start)
            return path[::-1]

        open_list.remove(current)
        closed_list.add(current)

        for neighbor in get_neighbors(current, grid):
            if neighbor in closed_list:
                continue

            tentative_g = g[current] + 1
            if neighbor not in open_list:
                open_list.append(neighbor)
            elif tentative_g >= g.get(neighbor, float('inf')):
                continue

            came_from[neighbor] = current
            g[neighbor] = tentative_g
            f[neighbor] = g[neighbor] + heuristic(neighbor, goal)

    return []

def heuristic(a, b):
    return abs(a[0] - b[0]) + abs(a[1] - b[1])

grid = [[0, 1, 0, 0],
        [0, 1, 0, 1],
        [0, 0, 0, 1],
        [1, 0, 0, 0]]

path = a_star((0, 0), (3, 3), grid)
print("Path:", path)
