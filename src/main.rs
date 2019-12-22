mod sort;
mod test;

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let vec = (0..10).map(|_| rng.gen()).collect::<Vec<u8>>();
    test::test_once_unsigned(&vec);
}
