pub trait Actuator {
    fn set_duty(&mut self, duty: f32);
}

// TODO: This isn't exactly the duty cycle, but more the range within the RC PWM. Rename accordingly.
const DEG_TO_DUTY: f32 = 1. / 180.;
const MIN_DUTY: f32 = 0.;
const MAX_DUTY: f32 = 1.;

// TODO: These will likely vary by physical implementation, so might need to be configurable.
const PITCH_OFFSET: f32 = 0.5;
const YAW_OFFSET: f32 = 0.5;

#[derive(Debug)]
pub struct Turret<A: Actuator> {
    pitch_motor: A,
    yaw_motor: A,
}

impl<A: Actuator> Turret<A> {
    pub fn new(pitch_motor: A, yaw_motor: A) -> Turret<A> {
        Turret {
            pitch_motor,
            yaw_motor,
        }
    }

    /// Sets the turret pitch angle, where -90 is straight down, +90 is straight up, and zero is horizontal.
    pub fn set_pitch_degrees(&mut self, deg: f32) {
        let duty = (deg * DEG_TO_DUTY + PITCH_OFFSET)
            .max(MIN_DUTY)
            .min(MAX_DUTY);
        self.pitch_motor.set_duty(duty);
    }

    /// Sets the turret yaw angle, where negative is left, positive is right and zero is straight forward.
    pub fn set_yaw_degrees(&mut self, deg: f32) {
        let duty = (deg * DEG_TO_DUTY + YAW_OFFSET)
            .max(MIN_DUTY)
            .min(MAX_DUTY);
        self.yaw_motor.set_duty(duty);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Derived from https://github.com/ohsayan/all_asserts.
    macro_rules! assert_near {
        ($left:expr, $right:expr $(,)?) => {{
            match (&$left, &$right) {
                (left_val, right_val) => {
                    let epsilon_val = 0.01;
                    if (*left_val > (*right_val + epsilon_val))
                        || (*left_val < (*right_val - epsilon_val))
                    {
                        panic!(
                            r#"assertion failed: `{} is not near {}`"#,
                            right_val, left_val
                        )
                    }
                }
            }
        }};
        ($left:expr, $right:expr, $($arg:tt)+ $(,)?) => {{
            match (&($left), &($right)) {
                (left_val, right_val) => {
                    let epsilon_val = 0.01;
                    if (*left_val > (*right_val + epsilon_val))
                        || (*left_val < (*right_val - epsilon_val))
                    {
                        panic!(
                            r#"assertion failed: `{} is not near {}`"#,
                            right_val, left_val
                        )
                    }
                }
            }
        }};
    }

    pub struct DummyActuator {
        duty: f32,
    }

    impl Actuator for DummyActuator {
        fn set_duty(&mut self, duty: f32) {
            self.duty = duty;
        }
    }

    #[test]
    fn test_new() {
        let t = Turret::new(DummyActuator { duty: 0.314 }, DummyActuator { duty: 0.999 });
        assert_near!(t.pitch_motor.duty, 0.314);
        assert_near!(t.yaw_motor.duty, 0.999);
    }

    #[test]
    fn test_pitch() {
        let mut t = Turret::new(DummyActuator { duty: 0.1 }, DummyActuator { duty: 0.5 });

        // Make sure pitch works.
        t.set_pitch_degrees(-99.);
        assert_near!(t.pitch_motor.duty, 0.);
        t.set_pitch_degrees(-89.);
        assert_near!(t.pitch_motor.duty, 1. / 180.);
        t.set_pitch_degrees(-45.);
        assert_near!(t.pitch_motor.duty, 0.25);
        t.set_pitch_degrees(0.);
        assert_near!(t.pitch_motor.duty, 0.5);
        t.set_pitch_degrees(45.);
        assert_near!(t.pitch_motor.duty, 0.75);
        t.set_pitch_degrees(90.);
        assert_near!(t.pitch_motor.duty, 1.);
        t.set_pitch_degrees(100.);
        assert_near!(t.pitch_motor.duty, 1.);

        // Make sure yaw wasn't affected.
        assert_near!(t.yaw_motor.duty, 0.5);
    }

    #[test]
    fn test_yaw() {
        let mut t = Turret::new(DummyActuator { duty: 0.25 }, DummyActuator { duty: 0.3 });

        // Make sure yaw works.
        t.set_yaw_degrees(-99.);
        assert_near!(t.yaw_motor.duty, 0.);
        t.set_yaw_degrees(-89.);
        assert_near!(t.yaw_motor.duty, 1. / 180.);
        t.set_yaw_degrees(-45.);
        assert_near!(t.yaw_motor.duty, 0.25);
        t.set_yaw_degrees(0.);
        assert_near!(t.yaw_motor.duty, 0.5);
        t.set_yaw_degrees(45.);
        assert_near!(t.yaw_motor.duty, 0.75);
        t.set_yaw_degrees(90.);
        assert_near!(t.yaw_motor.duty, 1.);
        t.set_yaw_degrees(100.);
        assert_near!(t.yaw_motor.duty, 1.);

        // Make sure pitch wasn't affected.
        assert_near!(t.pitch_motor.duty, 0.25);
    }
}
