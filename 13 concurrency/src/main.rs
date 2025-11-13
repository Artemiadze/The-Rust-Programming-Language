mod threading;
mod channels;
mod shared_state;

fn main() {
    threading::threads();
    println!("----------------");
    channels::example_channels();
    println!("----------------");
    shared_state::shared();
}