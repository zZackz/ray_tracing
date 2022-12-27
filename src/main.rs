#[derive(Copy, Clone, Debug)]
struct Vec3 {
    e: [f32; 3],
}

impl Vec3 {
    fn new(e0: f32, e1: f32, e2: f32) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }

    fn x(&self) -> f32 {
        self.e[0]
    }

    fn y(&self) -> f32 {
        self.e[1]
    }

    fn z(&self) -> f32 {
        self.e[2]
    }

    pub fn r(&self) -> f32 {
        self.e[0]
    }
    pub fn g(&self) -> f32 {
        self.e[1]
    }
    pub fn b(&self) -> f32 {
        self.e[2]
    }

    fn length_squared(&self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    fn length(&self) -> f32 {
        f32::sqrt(self.length_squared())
    }

    pub fn dot(v1: &Vec3, &v2: &Vec3) -> f32 {
        v1.e[0] * v2.e[0] + v1.e[1] * v2.e[1] + v1.e[2] * v2.e[2]
    }

    pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
        Vec3::new(
            v1.e[1] * v2.e[2] - v1.e[2] * v2.e[1],
            -(v1.e[0] * v2.e[2] - v1.e[2] * v2.e[0]),
            v1.e[0] * v2.e[1] - v1.e[1] * v2.e[0],
        )
    }

    pub fn make_unit_vector(v: Vec3) -> Vec3 {
        v / v.length()
    }
}

impl std::cmp::PartialEq for Vec3 {
    fn eq(&self, other: &Vec3) -> bool {
        f32::abs(self.e[0] - other.e[0]) < core::f32::EPSILON
            && f32::abs(self.e[1] - other.e[1]) < core::f32::EPSILON
            && f32::abs(self.e[2] - other.e[2]) < core::f32::EPSILON
    }

    fn ne(&self, other: &Vec3) -> bool {
        !self.eq(other)
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.e[0], self.e[1], self.e[2])
    }
}

impl std::ops::Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        &self.e[index]
    }
}

impl std::ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        &mut self.e[index]
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(
            self.e[0] + other.e[0],
            self.e[1] + other.e[1],
            self.e[2] + other.e[2],
        )
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, other: Vec3) -> Self {
        Self::new(
            self.e[0] - other.e[0],
            self.e[1] - other.e[1],
            self.e[2] - other.e[2],
        )
    }
}

impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self::new(
            self.e[0] * other.e[0],
            self.e[1] * other.e[1],
            self.e[2] * other.e[2],
        )
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Self::Output {
        Self::new(self.e[0] * other, self.e[1] * other, self.e[2] * other)
    }
}

impl std::ops::Div<Vec3> for Vec3 {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self::new(
            self.e[0] / other.e[0],
            self.e[1] / other.e[1],
            self.e[2] / other.e[2],
        )
    }
}

impl std::ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f32) -> Self::Output {
        Self::new(self.e[0] / other, self.e[1] / other, self.e[2] / other)
    }
}

impl std::ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

impl std::ops::AddAssign<f32> for Vec3 {
    fn add_assign(&mut self, other: f32) {
        self.e[0] += other;
        self.e[1] += other;
        self.e[2] += other;
    }
}

impl std::ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        self.e[0] -= other.e[0];
        self.e[1] -= other.e[1];
        self.e[2] -= other.e[2];
    }
}

impl std::ops::SubAssign<f32> for Vec3 {
    fn sub_assign(&mut self, other: f32) {
        self.e[0] -= other;
        self.e[1] -= other;
        self.e[2] -= other;
    }
}

impl std::ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        self.e[0] *= other.e[0];
        self.e[1] *= other.e[1];
        self.e[2] *= other.e[2];
    }
}

impl std::ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other: f32) {
        self.e[0] *= other;
        self.e[1] *= other;
        self.e[2] *= other;
    }
}

impl std::ops::DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, other: Vec3) {
        self.e[0] /= other.e[0];
        self.e[1] /= other.e[1];
        self.e[2] /= other.e[2];
    }
}

impl std::ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, other: f32) {
        self.e[0] /= other;
        self.e[1] /= other;
        self.e[2] /= other;
    }
}

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3\n{} {}\n255\n", image_width, image_height);

    for i in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {i:}");
        for j in 0..image_width {
            let color = Vec3::new(
                j as f32 / (image_width - 1) as f32,
                i as f32 / (image_height - 1) as f32,
                0.25,
            );
            write_color(color)
        }
    }
}

fn write_color(pixel_color: Vec3) {
    println!(
        "{} {} {}",
        255.999 * pixel_color.x(),
        255.999 * pixel_color.y(),
        255.999 * pixel_color.z()
    )
}
