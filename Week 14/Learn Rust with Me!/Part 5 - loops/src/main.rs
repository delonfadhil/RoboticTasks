fn main() {
    // Loop, berjalan selamanya tanpa pernyataan break
    /*
    loop {
        println!("Halo!");
    }
    */

    // Menggunakan kondisi break untuk menghentikan loop.
    // Dengan break, kita dapat menghentikan pengulangan aksi di dalam loop
    // dan juga mengembalikan nilai pada titik break
    let mut counter = 1;
    let loop_stop = loop {
        counter *= 4;
        if counter > 100 {
            break counter;
        }
    };

    println!("Hentikan loop pada counter = {}", loop_stop);

    // While loop menggunakan ekspresi kondisi. Loop akan terus berjalan selama kondisinya benar.
    let mut num = 0;
    while num < 10 {
        println!("Halo di sana!");
        num = num + 1;
    }

    // For loop menggunakan iterator untuk memproses kumpulan item.
    let shopping_list = ["milk", "cheese", "bread", "apples"];

    // Nilai iterator diikat pada hasil dari metode iter().
    for item in shopping_list.iter() {
        println!("Item berikutnya dalam daftar belanja saya adalah {}", item);
    }

    // Cara mudah lain untuk membuat iterator adalah dengan menggunakan notasi rentang a..b
    // Iterator dimulai dari nilai 'a' dan berlanjut satu per satu hingga nilai 'b', tetapi tidak termasuk 'b'.
    for number in 0..10 {
        println!("Angka {}", number);
    }
}
