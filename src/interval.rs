pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub fn new(min: f32, max: f32) -> Interval {
        Interval { min, max }
    }
    pub fn contains(&self, x: f32) -> bool {
        return self.min <= x && x <= self.max;
    }
    pub fn surrounds(&self, x: f32) -> bool {
        return self.min < x && x < self.max;
    }
    pub fn clamp(&self, x: f32) -> f32 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        return x;
    }
}

pub const EMPTY: Interval = Interval { min: f32::INFINITY, max: f32::NEG_INFINITY };
pub const UNIVERSE: Interval = Interval { min: f32::NEG_INFINITY, max: f32::INFINITY };

#[test]
fn test_new() {
    let i = Interval::new(0.0, 1.0);
    assert_eq!(i.min, 0.0);
    assert_eq!(i.max, 1.0);
}

#[test]
fn test_contains() {
    let i = Interval::new(0.0, 1.0);
    assert_eq!(i.contains(0.0), true);
    assert_eq!(i.contains(0.5), true);
    assert_eq!(i.contains(1.1), false);
}

#[test]
fn test_surrounds() {
    let i = Interval::new(0.0, 1.0);
    assert_eq!(i.surrounds(0.0), false);
    assert_eq!(i.surrounds(0.5), true);
    assert_eq!(i.surrounds(1.1), false);
}

#[test]
fn test_clamp() {
    let i = Interval::new(0.0, 1.0);
    assert_eq!(i.clamp(-1.0), 0.0);
    assert_eq!(i.clamp(2.0), 1.0);
    assert_eq!(i.clamp(0.5), 0.5);
}
