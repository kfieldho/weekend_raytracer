use std::ops::Neg;
use std::ops::Index;
use std::ops::AddAssign;
use std::ops::SubAssign;
use std::ops::MulAssign;
use std::ops::DivAssign;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

pub struct Vec3 {
    e: [f32; 3]
    }

//
// Methods
// 
impl Vec3 {
    fn origin() -> Vec3 {
        Vec3{ e: [ 0.0, 0.0, 0.0]}
    }
    pub fn new(x: f32, y: f32, z:f32) -> Vec3 {
        Vec3{ e: [x, y, z]}                 
    }

    // This implementation iis following the book's design.
    // Later I'll try to do a more idiomatic pass using traits 
    fn r(self) -> f32 {
        self.e[0]
    }
    fn g(self) -> f32 {
        self.e[1]
    }
    fn b(self) -> f32 {
        self.e[2]
    }
    fn x(self) -> f32 {
        self.e[0]
    }
    fn y(self) -> f32 {
        self.e[1]
    }
    fn z(self) -> f32 {
        self.e[2]
    }

    fn length(self) -> f32 {
        (self.e[0] * self.e[0] + self.e[1] * self.e[1]  + self.e[2] * self.e[2])
    }

    fn squared_length(self) -> f32 {
        (self.e[0] * self.e[0] + self.e[1] * self.e[1]  + self.e[2] * self.e[2])
    }

    fn make_unit_vector(mut self) {
        let k: f32 = 1.0 / (self.e[0] * self.e[0] + self.e[1] * self.e[1]  + self.e[2] * self.e[2]).sqrt();
        self.e[0] *= k;
        self.e[1] *= k;
        self.e[2] *= k;
    }
}


//
// Operators
// 

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3{e: [-self.e[0], -self.e[1], -self.e[2]]}
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self,index: usize) -> &f32 {
        &self.e[index]
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = Vec3{e: [self.e[0] + other.e[0], self.e[1] + other.e[1], self.e[2] + other.e[2]]};
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        *self = Vec3{e: [self.e[0] - other.e[0], self.e[1] - other.e[1], self.e[2] - other.e[2]]};
    }
}

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        *self = Vec3{e: [self.e[0] * other.e[0], self.e[1] * other.e[1], self.e[2] * other.e[2]]};
    }
}

impl DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, other: Vec3) {
        *self = Vec3{e: [self.e[0] / other.e[0], self.e[1] / other.e[1], self.e[2] / other.e[2]]};
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other: f32) {
        *self = Vec3{e: [self.e[0] * other, self.e[1] * other, self.e[2] * other]};
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, other: f32) {
        *self = Vec3{e: [self.e[0] / other, self.e[1] / other, self.e[2] / other]};
    }
}


impl Add for Vec3 {
    type Output = Vec3;

    fn add(self,other: Vec3) -> Vec3 {
        Vec3{e: [self.e[0] + other.e[0], 
                 self.e[1] + other.e[1], 
                 self.e[2] + other.e[2]]}
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self,other: Vec3) -> Vec3 {
        Vec3{e: [self.e[0] - other.e[0], 
                 self.e[1] - other.e[1], 
                 self.e[2] - other.e[2]]}
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self,other: Vec3) -> Vec3 {
        Vec3{e: [self.e[0] * other.e[0], 
                 self.e[1] * other.e[1], 
                 self.e[2] * other.e[2]]}
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self,other: f32) -> Vec3 {
        Vec3{e: [self.e[0] * other, 
                 self.e[1] * other, 
                 self.e[2] * other]}
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self,other: Vec3) -> Vec3 {
        Vec3{e: [self.e[0] / other.e[0], 
                 self.e[1] / other.e[1], 
                 self.e[2] / other.e[2]]}
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self,other: f32) -> Vec3 {
        Vec3{e: [self.e[0] / other, 
                 self.e[1] / other, 
                 self.e[2] / other]}
    }
}


//
// Functions
// 
fn dot(v1: Vec3, v2: Vec3) -> f32 {
    v1.e[0] * v2.e[0] + v1.e[1] * v2.e[1] + v1.e[2] + v2.e[2]    
}

fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
    Vec3{e: [
            v1.e[1] * v2.e[2] - v2.e[2] * v2.e[1],
            -(v1.e[0] * v2.e[2] - v2.e[2] * v2.e[0]),
            v1.e[0] * v2.e[1] - v2.e[1] * v2.e[0],
            ]
        }
}
