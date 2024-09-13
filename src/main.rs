use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let mut rng = thread_rng();
    let mut y = [1, 2, 3, 4, 5];
    println!("Unshuffled: {:?}", y);
    y.shuffle(&mut rng);
    println!("Shuffled:   {:?}", y);
}
