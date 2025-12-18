mod insertion_sort;
use rand::Rng; 

fn main() {
    // Run the thing here
    let mut vec: Vec<i32>  = vec![1, 5, 10, 2, 15];

    {
        things::insertion_sort(&vec);
    }
}
