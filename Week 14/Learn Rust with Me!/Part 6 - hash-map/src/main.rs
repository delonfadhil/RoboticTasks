fn main() {
    
    // Perintah 'use' membawa definisi HashMap dari bagian 'collections' di pustaka standar Rust.
    use std::collections::HashMap;
    // Kita membuat hash map kosong menggunakan metode HashMap::new().
    let mut items: HashMap<String, String> = HashMap::new();

    // Kita menambahkan elemen ke hash map dengan menggunakan metode insert(<key>, <value>).
    items.insert(String::from("One"), String::from("Book"));
    items.insert(String::from("Two"), String::from("Keyboard"));
    items.insert(String::from("Three"), String::from("Sunglasses"));

    // Setelah menambahkan data ke hash map, kita bisa mengambil nilai tertentu berdasarkan kuncinya dengan metode get(<key>).
    let keyboard = items.get("Two");
    println!("{:?}", keyboard);

    // Kita juga bisa menghapus entri dari hash map menggunakan metode .remove(<key>).
    // Jika kita mencoba mengambil nilai yang sudah dihapus dari hash map, 'None' akan dikembalikan.
    items.remove("Three");
    
    println!("{:?}", items.get("Three"));
}
