use vec3;

pub struct Ray {
    A: vec3::Vec3,
    B: vec3::Vec3
}

impl Ray {
    pub fn new(a: vec3::Vec3, b: vec3::Vec3) -> Ray {
        Ray{ A: a, B: b}
    }

    pub fn origin(self) -> vec3::Vec3 {
        self.A
    }

    pub fn direction(self) -> vec3::Vec3 {
        self.B
    }

    pub fn point_at_parameter(self, t: f32) -> vec3::Vec3
    {
        self.A + t * self.B
    }

}

