use std::collections::VecDeque;

fn is_valid_move(x: i32, y: i32, rows: usize, cols: usize, grid: &Vec<Vec<i32>>, visited: &Vec<Vec<bool>>) -> bool {
    x >= 0 && y >= 0 && x < rows as i32 && y < cols as i32 && grid[x as usize][y as usize] == 0 && !visited[x as usize][y as usize]
}

fn find_path(grid: Vec<Vec<i32>>) -> Option<Vec<(i32, i32)>> {
    let rows = grid.len();
    let cols = grid[0].len();
    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut visited = vec![vec![false; cols]; rows];
    let mut queue: VecDeque<(i32, i32, Vec<(i32, i32)>)> = VecDeque::new();

    // Mulai dari titik awal (0, 0)
    queue.push_back((0, 0, vec![(0, 0)]));
    visited[0][0] = true;

    while let Some((x, y, path)) = queue.pop_front() {
        // Jika sampai di tujuan (baris terakhir, kolom terakhir)
        if x == (rows as i32 - 1) && y == (cols as i32 - 1) {
            return Some(path);
        }

        for (dx, dy) in &directions {
            let nx = x + dx;
            let ny = y + dy;

            if is_valid_move(nx, ny, rows, cols, &grid, &visited) {
                let mut new_path = path.clone();
                new_path.push((nx, ny));
                queue.push_back((nx, ny, new_path));
                visited[nx as usize][ny as usize] = true;
            }
        }
    }

    // Jika tidak ada jalur yang ditemukan
    None
}

fn main() {
    let grid = vec![
        vec![0, 0, 0, 0, 0],
        vec![1, 1, 0, 1, 1],
        vec![0, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 0],
        vec![0, 0, 0, 0, 0],
    ];

    println!("Peta:");
    for row in &grid {
        println!("{:?}", row);
    }

    match find_path(grid) {
        Some(path) => {
            println!("Jalur ditemukan:");
            for (i, (x, y)) in path.iter().enumerate() {
                println!("Langkah {}: ({}, {})", i + 1, x, y);
            }
        }
        None => {
            println!("Tidak ada jalur yang ditemukan.");
        }
    }
}
