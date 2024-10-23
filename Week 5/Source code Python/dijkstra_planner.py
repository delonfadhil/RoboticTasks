import heapq

def dijkstra(graph, start, goal):
    queue = [(0, start, [])]
    seen = set()
    min_distance = {start: 0}
    
    while queue:
        (cost, v1, path) = heapq.heappop(queue)
        if v1 in seen:
            continue

        path = path + [v1]
        if v1 == goal:
            return cost, path

        seen.add(v1)
        for v2, c in graph[v1].items():
            if v2 in seen:
                continue
            prev = min_distance.get(v2, None)
            next_cost = cost + c
            if prev is None or next_cost < prev:
                min_distance[v2] = next_cost
                heapq.heappush(queue, (next_cost, v2, path))

    return float("inf"), []

graph = {
    'A': {'B': 1, 'C': 4},
    'B': {'A': 1, 'C': 2, 'D': 5},
    'C': {'A': 4, 'B': 2, 'D': 1},
    'D': {'B': 5, 'C': 1}
}

cost, path = dijkstra(graph, 'A', 'D')
print("Cost:", cost)
print("Path:", path)