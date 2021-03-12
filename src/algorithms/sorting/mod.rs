mod quick_sort;


pub use self::util::rand_arr;
mod util {
    extern crate rand;
    use self::rand::random;

    const RAND_SIZE: usize = 100;
    pub fn rand_arr() -> [i32; RAND_SIZE] {
        let mut arr = [0; RAND_SIZE];
        for x in arr.iter_mut() {
            *x = random::<i32>()
        }
        arr
    }
}