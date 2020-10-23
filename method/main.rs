/*
 * seperti bahasa oop lain yang memiliki method
 * di rust juga bisa membuat method
 * bedanya jika kita memanggil method harus membuat instance
 * dari class, di rust kita harus membuat instance dari struct
 *
 * */

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// implementation block
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// create instance from Rectangle struct
fn new_rectangle() -> Rectangle {
    return Rectangle{
        width: 13,
        height: 10,
    };
}


fn main() {
    
    // create instance
    let rect = new_rectangle();

    println!("the area of reactangle is {}", rect.area());
}
