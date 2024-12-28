// Struct klasik dengan nama field dan tipe datanya
struct Student {
    name: String,
    level: u8,
    remote: bool
}

// Tuple struct hanya berisi tipe data
struct Grades(char,char,char, f32);

// Fungsi utama program, dimulai dengan keyword 'fn' untuk mendeklarasikan fungsi
fn main() {

    // println! adalah macro bawaan Rust untuk mencetak output ke konsol
    println!("Hello, {} {}!", "Will", "Velida");

    // Variabel bersifat immutable (tidak bisa diubah) secara default
    // Gunakan keyword 'mut' agar variabel bisa diubah nilainya
    let mut age = 33;
    let birth_year = 1991;

    println!("I am {} years old", age);

    // Variabel shadowing (membayangi variabel sebelumnya)
    let birth_year = birth_year - 1;
    
    age = 34;

    println!("I am now {} years old", age);
    println!("I was born in {}", birth_year);

    // Rust adalah bahasa dengan tipe data statis. Tipe data harus jelas saat kompilasi
    let nephew_age: u32 = 14;
    println!("My nephew is {} years old", nephew_age);

    // Float
    let float: f32 = 4.0;
    
    println!("1 x 2 = {}", 1*2);

    // Boolean: hanya bernilai true atau false
    let is_bigger_num = 2 < 4;
    println!("Is 2 < 4: {}", is_bigger_num);

    // Strings
    // Karakter: tipe data untuk satu karakter
    let first_char: char = 'W';
    let last_char: char = 'l';

    let second_char = 'i';

    // String adalah kumpulan karakter
    let my_name = "Will";

    println!("{} is the first character, {} is the last character, {} is the second character of my name {}", first_char, last_char, second_char, my_name);

    // Tuple: gabungan nilai dengan tipe data berbeda. Panjangnya tetap
    let my_dog = ("Toby", 15, false);

    println!("My dog's name was {}, he was {} years old, is he alive? {}", my_dog.0, my_dog.1, my_dog.2);
    
    // Membuat instance struct dengan mengisi nilai field
    let student_1 = Student{
        name: String::from("Will Velida"),
        remote: true,
        level: 5
    };

    let grades = Grades('A','A','B',3.5);

    // Struct klasik: akses nilai menggunakan nama field
    println!("{}, is a level {} programmer. Does he work remotely: {}",
        student_1.name, student_1.level, student_1.remote);

    // Tuple struct: akses nilai menggunakan indeks
    println!("{},{},{},GPA = {}", grades.0, grades.1, grades.2, grades.3);
}