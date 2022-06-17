use linux_embedded_hal::I2cdev;
use pwm_pca9685::{Address, Channel, Pca9685};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

mod sphero;
mod turret;

//#[derive(Debug)]
pub struct RealActuator {
    pwm: Arc<Mutex<Pca9685<I2cdev>>>,
    chan: Channel,
}

impl RealActuator {
    pub fn new(pwm: Arc<Mutex<Pca9685<I2cdev>>>, chan: Channel) -> RealActuator {
        RealActuator { pwm, chan }
    }
}

impl turret::Actuator for RealActuator {
    fn set_pulse_ms(&mut self, ms: f32) {
        // With a prescale of 100, we get approximately 58Hz.
        // Empirical tests gave a value of around 16.96ms to 16.98ms.
        let period_usec = 16_970_f32;
        // The pulse period is 4096 ticks.
        let period_ticks = 4_096_f32;

        let pulse_usec = ms * 1_000_f32;
        let pulse_ticks = (pulse_usec * period_ticks / period_usec).round() as u16;

        let mut pwm = self.pwm.lock().unwrap();

        println!("Ticks: {}", pulse_ticks);

        // Pulse is on for ticks [0..on_ticks-1] and then off from [on_ticks..4095].
        pwm.set_channel_on(self.chan, 0).unwrap();
        pwm.set_channel_off(self.chan, pulse_ticks).unwrap();
    }
}

fn init_pwm() -> Arc<Mutex<Pca9685<I2cdev>>> {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let address = Address::default();
    let mut pwm = Pca9685::new(dev, address).unwrap();
    let servo_60hz_prescale = 100;
    pwm.set_prescale(servo_60hz_prescale).unwrap();
    pwm.enable().unwrap();

    Arc::new(Mutex::new(pwm))
}

fn disable_pwm(pwm: Arc<Mutex<Pca9685<I2cdev>>>) {
    let mut pwm = pwm.lock().unwrap();
    pwm.disable().unwrap();
}

fn main() {
    let pwm = init_pwm();

    let mut t = turret::Turret::new(
        1. / 90., // The turret servos seem to require twice a normal RC servo.
        RealActuator::new(pwm.clone(), Channel::C0),
        RealActuator::new(pwm.clone(), Channel::C1),
    );

    t.set_pitch_degrees(60.);
    for i in -9..=9 {
        let deg = i as f32 * 10.;
        t.set_yaw_degrees(deg);
        thread::sleep(Duration::from_millis(1000));
    }

    disable_pwm(pwm);
}
