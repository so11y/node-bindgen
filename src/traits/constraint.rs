#[derive(Debug, Clone)]
pub struct Constraint {
    max_width: f64,
    max_height: f64,
    x: f64,
    y: f64,
}

impl Default for Constraint {
    fn default() -> Self {
        Self {
            max_width: 0.0,
            max_height: 0.0,
            x: 0.0,
            y: 0.0,
        }
    }
}
impl Constraint {
    pub fn new(max_width: f64, max_height: f64, x: f64, y: f64) -> Self {
        Self {
            max_width,
            max_height,
            x,
            y,
        }
    }
    pub fn maybe_constraint(&self, width: f64, height: f64) -> bool {
        return width <= self.max_width && height <= self.max_height;
    }
}
