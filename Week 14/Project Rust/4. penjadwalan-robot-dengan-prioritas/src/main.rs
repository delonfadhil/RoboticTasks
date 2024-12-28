use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[derive(Debug, Eq, PartialEq)]
struct Task {
    priority: u32,
    description: String,
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.priority.cmp(&self.priority)
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut task_queue = BinaryHeap::new();

    // Menambahkan tugas ke antrean dengan prioritas
    task_queue.push(Task {
        priority: 1,
        description: String::from("Ambil barang dari lokasi A"),
    });

    task_queue.push(Task {
        priority: 3,
        description: String::from("Periksa sistem keamanan"),
    });

    task_queue.push(Task {
        priority: 2,
        description: String::from("Bawa barang ke lokasi B"),
    });

    println!("Mulai menyelesaikan tugas berdasarkan prioritas:");

    // Robot menyelesaikan tugas berdasarkan prioritas tertinggi
    while let Some(task) = task_queue.pop() {
        println!("Menyelesaikan tugas: {} dengan prioritas {}", task.description, task.priority);
    }
}
