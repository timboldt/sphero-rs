mod sphero;

fn main() {
    let _ = sphero::Packet::new(sphero::Device::SomeDevice1, sphero::Command::SomeCommand1, 22, vec![1, 2, 3]);
    println!("Hello, world!");
}
