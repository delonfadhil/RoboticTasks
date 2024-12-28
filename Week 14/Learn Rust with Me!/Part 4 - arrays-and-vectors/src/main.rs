fn main() {

    // Array: Kumpulan objek dengan tipe data yang sama disimpan secara berurutan dalam memori
    // Kita bisa mendeklarasikan array, menginisialisasi semua nilainya, dan kompilator akan menebak panjangnya
    let working_days = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];

    // Kita juga bisa mendeklarasikan array, menginisialisasi semua nilainya, dan menentukan panjangnya
    let working_days_num = [0; 5];

    // Kita juga bisa mengakses elemen dalam array menggunakan indeks posisinya
    println!("{}", working_days[0]);

    // Vektor juga menyimpan banyak nilai dengan tipe data yang sama
    // Kita bisa mendeklarasikan vektor dan menginisialisasi semua nilainya
    let nephews_age = vec![14, 9, 0];
    println!("Usia keponakan: {:?}", nephews_age);

    // Kita juga bisa mendeklarasikan vektor, menginisialisasi semua nilainya, dan menentukan panjangnya
    let zeroes = vec![0; 5];
    println!("Zeroes: {:?}", zeroes);

    // Kita bisa menambahkan atau menghapus nilai dari vektor menggunakan metode push atau pop
    let mut names = Vec::new();

    names.push("Will");
    names.push("Isaac");
    names.push("Sam");

    println!("Nama: {:?}", names);

    names.pop();
    println!("Nama: {:?}", names);

    // Kita juga bisa mengakses elemen dalam vektor berdasarkan posisinya di vektor
    let mut fruit = vec!["Apple", "Melon", "Orange"];
    let orange = fruit[2];
    fruit[0] = "Strawberry";
    println!("Buah: {:?}, Orange = {}", fruit, orange);
    
}
