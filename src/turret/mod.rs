pub trait Actuator {
    fn set_duty(&mut self, duty: f32);
}

const PITCH_SCALE: f32 = 1. / 180.;
const PITCH_OFFSET: f32 = 0.5;
const PITCH_MIN_DUTY: f32 = 0.;
const PITCH_MAX_DUTY: f32 = 1.;
const YAW_SCALE: f32 = 1. / 180.;
const YAW_OFFSET: f32 = 0.5;
const YAW_MIN_DUTY: f32 = 0.;
const YAW_MAX_DUTY: f32 = 1.;

#[derive(Debug)]
pub struct Turret<A: Actuator> {
    pitch_motor: A,
    yaw_motor: A,
}

impl<A: Actuator> Turret<A> {
    fn new(pitch_motor: A, yaw_motor: A) -> Turret<A> {
        Turret {
            pitch_motor,
            yaw_motor,
        }
    }

    /// Sets the turret pitch angle, where -90 is straight down, +90 is straight up, and zero is horizontal.
    fn set_pitch_degrees(&mut self, deg: f32) {
        let duty = (deg * PITCH_SCALE + PITCH_OFFSET).max(PITCH_MIN_DUTY).min(PITCH_MAX_DUTY);
        self.pitch_motor.set_duty(duty);
    }

    /// Sets the turret yaw angle, where negative is left, positive is right and zero is straight forward.
    fn set_yaw_degrees(&mut self, deg: f32) {
        let duty = (deg * YAW_SCALE + YAW_OFFSET).max(YAW_MIN_DUTY).min(YAW_MAX_DUTY);
        self.yaw_motor.set_duty(duty);
    }
}
