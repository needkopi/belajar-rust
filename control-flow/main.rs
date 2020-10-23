fn main() {

    /*
     * if expression
     *
     * */

    let number = 3;

    if number < 5 {
        println!("variabel number kurang dari 5");
    } else if number == 3 {
        println!("variabel number sama dengan 3");
    } else {
        println!("variabel number lebih dari 5");
    }


    /*
     * nilai variable dengan if statment
     *
     * */
    let yes = true;
    let a = if yes { 10 } else { 0 };

    println!("value of a is: {}",a);

    /*
     * nilai variable dengan loop
     *
     * */

    let mut counter = 0;

    let b = loop {
        counter += 1;

        if counter == 3 {
            break counter * 10;
        }
    };

    println!("value of b is: {}", b);

    /*
     *  foreach or range array
     * 
     * */

    let arr = [1,2,3,4,5];

    for ell in arr.iter() {
        println!("ellement is: {}", ell);
    }

    // OR

    for val in (1..6).rev() {
        println!("value is {}", val);
    }
}
