use rand::Rng;

fn main() {

    let mut rng = rand::thread_rng();
    println!("Your lucky number today is : {}", rng.gen_range(0, 10));
}
