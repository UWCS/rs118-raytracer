pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Point = Vec3;
pub type Colour = Vec3;

impl Vec3 {
    pub fn len(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalise(self) -> Self {
        let l = self.len();
        self.map(|x| x / l)
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn to_rgb(self) -> image::Rgb<u8> {
        image::Rgb(
            [self.x, self.y, self.z]
                .map(f64::sqrt)
                .map(|c| (c * 255.999) as u8),
        )
    }

    pub fn map<F>(self, mut f: F) -> Vec3
    where
        F: FnMut(f64) -> f64,
    {
        Vec3 {
            x: f(self.x),
            y: f(self.y),
            z: f(self.z),
        }
    }
}
