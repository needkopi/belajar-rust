fn main() {

    /*
     * secara default variabel adalah immutable.
     * tetapi tetap bisa dibuat mutalble dengan menambahkan 
     * keyworld mut
     *
     * example
     *  let a  <- immutable
     *  let mut b <- mutable
     *
     * */

    let a = 5;
    println!("value if a is: {}", a);
    /*
     * jika diganti nilainya, misal
     * a = 6;
     * itu akan mengembalikan error
     * "cannot assign twice to immutable variable `a`"
     *
     * */

    let mut b = 5;
    println!("value of b is: {}", b);

    b = 6;
    println!("value of b is: {}", b);

    /*
     * CONSTANTA
     *
     * perbedaan constanta dengan immuatable variabel
     * 1. constanta tidak bisa dijadikan mutable atau 
     *    diberi tag/keyword mut
     *
     * 2. deklarasi constanta dengan `const` sedangkan variable
     *    dengan `let`, dan type data harus didefinisikan.
     *
     * 3. constanta bisa di deklarasikan dimanapun (global)
     *
     * 4. constanta adalah deklarasi konstan, bukan dari hasil fungsi 
     *    atau nilai runtime yang lain
     *
     *
     * example constanta
     *
     * const MAX_POINTS: u32 = 100_000;
     * */


}
