use std::io;

fn main() {
    let mut position = (0, 0); // Posisi awal robot (x, y)

    println!("Posisi awal robot: ({}, {})", position.0, position.1);

    loop {
        println!("Masukkan perintah untuk menggerakkan robot (up, down, left, right) atau 'exit' untuk keluar:");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Gagal membaca input");

        let command = input.trim();

        match command {
            "up" => position.1 += 1,
            "down" => position.1 -= 1,
            "left" => position.0 -= 1,
            "right" => position.0 += 1,
            "exit" => {
                println!("Keluar dari program.");
                break;
            }
            _ => println!("Perintah tidak dikenal. Gunakan 'up', 'down', 'left', 'right', atau 'exit'"),
        }

        println!("Posisi robot saat ini: ({}, {})", position.0, position.1);
    }
}
