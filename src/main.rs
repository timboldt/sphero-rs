mod sphero;
mod turret;

#[derive(Debug)]
pub struct NotYetRealActuator {
    duty: f32,
}

impl turret::Actuator for NotYetRealActuator {
    fn set_duty(&mut self, duty: f32) {
        self.duty = duty;
    }
}

fn main() {
    let _ = sphero::Packet::new(
        sphero::Device::SomeDevice1,
        sphero::Command::SomeCommand1,
        vec![1, 2, 3],
    );
    let mut t = turret::Turret::new(NotYetRealActuator { duty: 0.5 }, NotYetRealActuator { duty: 0.5 });
    t.set_pitch_degrees(0.);
    println!("{:?}", t);
}
