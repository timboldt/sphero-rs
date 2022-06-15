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
    duty: f32,
}

impl RealActuator {
    pub fn new(pwm: Arc<Mutex<Pca9685<I2cdev>>>, chan: Channel) -> RealActuator {
        RealActuator {
            pwm,
            chan,
            duty: 0.,
        }
    }
}

impl turret::Actuator for RealActuator {
    fn set_duty(&mut self, duty: f32) {
        self.duty = duty;

        // With a prescale of 100, we get approximately 58Hz.
        // Empirical tests gave a value of around 16.96ms to 16.98ms.
        let period_usec = 16_970_f32;
        // The pulse period is 4096 ticks.
        let period_ticks = 4_096_f32;

        // RC servos use pulses from 1000us to 2000us.
        let pulse_usec = duty * 1_000_f32 + 1_000_f32;
        let pulse_ticks = (pulse_usec * period_ticks / period_usec).round() as u16;

        let mut pwm = self.pwm.lock().unwrap();

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

fn main() {
    // let _ = sphero::Packet::new(
    //     sphero::Device::SomeDevice1,
    //     sphero::Command::SomeCommand1,
    //     vec![1, 2, 3],
    // );

    let pwm = init_pwm();
    
    let mut t = turret::Turret::new(
        RealActuator::new(pwm.clone(), Channel::C0),
        RealActuator::new(pwm.clone(), Channel::C1),
    );
    t.set_pitch_degrees(-90.);
    t.set_yaw_degrees(90.);

    thread::sleep(Duration::from_millis(100));

    let mut pwm = pwm.lock().unwrap();
    pwm.disable().unwrap();
}
