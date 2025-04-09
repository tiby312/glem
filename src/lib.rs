use glam::f32::Mat4;


pub mod prelude {
    pub use super::Mat;
}

pub fn build_inverse(a: impl Inverse) -> Mat4 {
    let mut m = Mat4::IDENTITY;
    a.apply_inverse(&mut m);
    m
}

pub fn build(a: impl Mat) -> Mat4 {
    let mut m = Mat4::IDENTITY;
    a.apply(&mut m);
    m
}

pub trait Inverse: Mat {
    fn apply_inverse(&self, a: &mut Mat4);
}

impl<T: Inverse> Inverse for &T {
    fn apply_inverse(&self, m: &mut Mat4) {
        (**self).apply_inverse(m)
    }
}

impl Mat for Mat4 {
    fn apply(&self, m: &mut Mat4) {
        *m *= *self;
    }
}

impl Inverse for Mat4 {
    fn apply_inverse(&self, a: &mut Mat4) {
        *a *= self.inverse()
    }
}

impl<T: Mat> Mat for &T {
    fn apply(&self, m: &mut Mat4) {
        (**self).apply(m)
    }
}

pub trait Mat {
    fn apply(&self, m: &mut Mat4);

    fn chain<K: Mat>(self, other: K) -> Chain<Self, K>
    where
        Self: Sized,
    {
        Chain { a: self, b: other }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Chain<A, B> {
    a: A,
    b: B,
}
impl<A: Inverse, B: Inverse> Inverse for Chain<A, B> {
    fn apply_inverse(&self, a: &mut Mat4) {
        self.b.apply_inverse(a);
        self.a.apply_inverse(a);
    }
}
impl<A: Mat, B: Mat> Mat for Chain<A, B> {
    fn apply(&self, m: &mut Mat4) {
        self.a.apply(m);
        self.b.apply(m);
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Scale {
    pub tx: f32,
    pub ty: f32,
    pub tz: f32,
}

impl Inverse for Scale {
    fn apply_inverse(&self, m: &mut Mat4) {
        scale(1.0 / self.tx, 1.0 / self.ty, 1.0 / self.tz).apply(m)
    }
}
impl Mat for Scale {
    fn apply(&self, m: &mut Mat4) {
        *m *= Mat4::from_cols_array(&[
            self.tx, 0., 0., 0., 0., self.ty, 0., 0., 0., 0., self.tz, 0., 0., 0., 0., 1.0,
        ])
    }
}

#[derive(Copy, Clone, Debug)]
pub struct XRot {
    pub angle_rad: f32,
}
impl Inverse for XRot {
    fn apply_inverse(&self, m: &mut Mat4) {
        rotate_x(-self.angle_rad).apply(m)
    }
}
impl Mat for XRot {
    fn apply(&self, m: &mut Mat4) {
        let c = self.angle_rad.cos();
        let s = self.angle_rad.sin();

        *m *= Mat4::from_cols_array(&[1., 0., 0., 0., 0., c, s, 0., 0., -s, c, 0., 0., 0., 0., 1.])
    }
}

#[derive(Copy, Clone, Debug)]
pub struct YRot {
    pub angle_rad: f32,
}
impl Inverse for YRot {
    fn apply_inverse(&self, m: &mut Mat4) {
        rotate_y(-self.angle_rad).apply(m)
    }
}
impl Mat for YRot {
    fn apply(&self, m: &mut Mat4) {
        let c = self.angle_rad.cos();
        let s = self.angle_rad.sin();

        *m *= Mat4::from_cols_array(&[c, 0., -s, 0., 0., 1., 0., 0., s, 0., c, 0., 0., 0., 0., 1.])
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ZRot {
    pub angle_rad: f32,
}
impl Inverse for ZRot {
    fn apply_inverse(&self, m: &mut Mat4) {
        rotate_z(-self.angle_rad).apply(m)
    }
}
impl Mat for ZRot {
    fn apply(&self, m: &mut Mat4) {
        let c = self.angle_rad.cos();
        let s = self.angle_rad.sin();

        *m *= Mat4::from_cols_array(&[c, s, 0., 0., -s, c, 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.])
    }
}

pub fn rotate_x(angle_rad: f32) -> XRot {
    XRot { angle_rad }
}
pub fn rotate_y(angle_rad: f32) -> YRot {
    YRot { angle_rad }
}
pub fn rotate_z(angle_rad: f32) -> ZRot {
    ZRot { angle_rad }
}

pub fn scale(x: f32, y: f32, z: f32) -> Scale {
    Scale {
        tx: x,
        ty: y,
        tz: z,
    }
}
pub fn translate(tx: f32, ty: f32, tz: f32) -> Translation {
    Translation { tx, ty, tz }
}

#[derive(Copy, Clone, Debug)]
pub struct Translation {
    tx: f32,
    ty: f32,
    tz: f32,
}

impl Inverse for Translation {
    fn apply_inverse(&self, m: &mut Mat4) {
        translate(-self.tx, -self.ty, -self.tz).apply(m)
    }
}
impl Mat for Translation {
    fn apply(&self, m: &mut Mat4) {
        let tx = self.tx;
        let ty = self.ty;
        let tz = self.tz;
        *m *= Mat4::from_cols_array(&[
            1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., tx, ty, tz, 1.,
        ])
    }
}


#[macro_export]
macro_rules! combine {
    ($a:expr)=>{
        $a
    };
    ( $a:expr,$( $x:expr ),* ) => {
        {
            use $crate::Mat;
            let mut a=$a;
            $(
                let a=a.chain($x);
            )*

            a
        }
    };
}
