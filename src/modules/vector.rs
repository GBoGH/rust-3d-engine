use crate::misc::axis::Axis;

#[derive(Debug)]
pub struct Vector3(pub f32, pub f32, pub f32);
#[derive(Debug)]
pub struct Vector2(pub f32, pub f32);
#[derive(Debug)]
pub struct  Vector2Color(pub f32, pub f32, pub u8);




impl Vector3{
    pub fn rotate(&mut self, angle: f32, axis: &Axis){
        let (x, y, z) = (self.0, self.1, self.2);
        match axis {
            Axis::X => {
                self.0 = x;
                self.1 = y * angle.cos() - z * angle.sin();
                self.2 = y * angle.sin() + z * angle.cos();
            },
            Axis::Y => {
                self.0 = x * angle.cos() + z * angle.sin();
                self.1 = y;
                self.2 = -x * angle.sin() + z * angle.cos();
            },
            Axis::Z => {
                self.0 = x * angle.cos() - y * angle.sin();
                self.1 = x * angle.sin() + y * angle.cos();
                self.2 = z;
            }
        }
    }

    pub fn add(&mut self, vector: &Vector3){
        self.0 += vector.0;
        self.1 += vector.1;
        self.2 += vector.2;
    }
}