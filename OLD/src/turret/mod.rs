pub trait Actuator {
    fn set_pulse_ms(&mut self, pulse: f32);
}

const MIN_PULSE: f32 = 0.5;
const CENTER_PULSE: f32 = 1.5;
const MAX_PULSE: f32 = 2.5;

#[derive(Debug)]
pub struct Turret<A: Actuator> {
    deg_to_ms: f32,
    pitch_motor: A,
    yaw_motor: A,
}

impl<A: Actuator> Turret<A> {
    pub fn new(deg_to_ms: f32, pitch_motor: A, yaw_motor: A) -> Turret<A> {
        Turret {
            deg_to_ms,
            pitch_motor,
            yaw_motor,
        }
    }

    /// Sets the turret pitch angle, where -90 is straight down, +90 is straight up, and zero is horizontal.
    pub fn set_pitch_degrees(&mut self, deg: f32) {
        let pulse = (deg * self.deg_to_ms + CENTER_PULSE)
            .max(MIN_PULSE)
            .min(MAX_PULSE);
        self.pitch_motor.set_pulse_ms(pulse);
    }

    /// Sets the turret yaw angle, where negative is left, positive is right and zero is straight forward.
    pub fn set_yaw_degrees(&mut self, deg: f32) {
        let pulse = (deg * self.deg_to_ms + CENTER_PULSE)
            .max(MIN_PULSE)
            .min(MAX_PULSE);
        self.yaw_motor.set_pulse_ms(pulse);
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
        pulse: f32,
    }

    impl Actuator for DummyActuator {
        fn set_pulse_ms(&mut self, pulse: f32) {
            self.pulse = pulse;
        }
    }

    #[test]
    fn test_new() {
        let t = Turret::new(
            1. / 180.,
            DummyActuator { pulse: 0.314 },
            DummyActuator { pulse: 0.999 },
        );
        assert_near!(t.pitch_motor.pulse, 0.314);
        assert_near!(t.yaw_motor.pulse, 0.999);
    }

    #[test]
    fn test_pitch() {
        let mut t = Turret::new(
            1. / 180.,
            DummyActuator { pulse: 0.1 },
            DummyActuator { pulse: 0.5 },
        );

        // Make sure pitch works.
        t.set_pitch_degrees(-90.);
        assert_near!(t.pitch_motor.pulse, 1.0);
        t.set_pitch_degrees(-89.);
        assert_near!(t.pitch_motor.pulse, 1.0 + 1. / 180.);
        t.set_pitch_degrees(-45.);
        assert_near!(t.pitch_motor.pulse, 1.25);
        t.set_pitch_degrees(0.);
        assert_near!(t.pitch_motor.pulse, 1.5);
        t.set_pitch_degrees(45.);
        assert_near!(t.pitch_motor.pulse, 1.75);
        t.set_pitch_degrees(90.);
        assert_near!(t.pitch_motor.pulse, 2.0);
        t.set_pitch_degrees(200.);
        assert_near!(t.pitch_motor.pulse, MAX_PULSE);

        // Make sure yaw wasn't affected.
        assert_near!(t.yaw_motor.pulse, 0.5);
    }

    #[test]
    fn test_yaw() {
        let mut t = Turret::new(
            1. / 180.,
            DummyActuator { pulse: 0.25 },
            DummyActuator { pulse: 0.3 },
        );

        // Make sure yaw works.
        t.set_yaw_degrees(-200.);
        assert_near!(t.yaw_motor.pulse, MIN_PULSE);
        t.set_yaw_degrees(-90.);
        assert_near!(t.yaw_motor.pulse, 1.0);
        t.set_yaw_degrees(-45.);
        assert_near!(t.yaw_motor.pulse, 1.25);
        t.set_yaw_degrees(0.);
        assert_near!(t.yaw_motor.pulse, 1.5);
        t.set_yaw_degrees(45.);
        assert_near!(t.yaw_motor.pulse, 1.75);
        t.set_yaw_degrees(90.);
        assert_near!(t.yaw_motor.pulse, 2.);

        // Make sure pitch wasn't affected.
        assert_near!(t.pitch_motor.pulse, 0.25);
    }
}
