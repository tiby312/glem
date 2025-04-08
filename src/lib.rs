use glam::f32::Mat4;

pub mod prelude {
    pub use super::Mat;
}

pub fn build_inverse(a: &impl Inverse) -> Mat4 {
    let mut m = Mat4::IDENTITY;
    a.apply_inverse(&mut m);
    m
}

pub fn build(a: &impl Mat) -> Mat4 {
    let mut m = Mat4::IDENTITY;
    a.apply(&mut m);
    m
}

pub trait Inverse: Mat {
    type Neg: Mat;
    fn generate_inverse(&self) -> Self::Neg;
    fn apply_inverse(&self, a: &mut Mat4) {
        *a *= self.generate_inverse().generate();
    }
}

impl Mat for Mat4 {
    fn generate(&self) -> Mat4 {
        self.clone()
    }
}

impl Inverse for Mat4 {
    type Neg = Self;
    fn generate_inverse(&self) -> Self::Neg {
        self.inverse()
    }
}
pub trait Mat {
    fn generate(&self) -> Mat4;
    fn apply(&self, foo: &mut Mat4) {
        *foo *= self.generate();
    }
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
    type Neg = Chain<B::Neg, A::Neg>;
    fn generate_inverse(&self) -> Self::Neg {
        Chain {
            a: self.b.generate_inverse(),
            b: self.a.generate_inverse(),
        }
    }
    fn apply_inverse(&self, a: &mut Mat4) {
        self.b.apply_inverse(a);
        self.a.apply_inverse(a);
    }
}
impl<A: Mat, B: Mat> Mat for Chain<A, B> {
    fn apply(&self, foo: &mut Mat4) {
        self.a.apply(foo);
        self.b.apply(foo);
    }
    fn generate(&self) -> Mat4 {
        let a = self.a.generate();
        let b = self.b.generate();
        a * b
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Scale {
    pub tx: f32,
    pub ty: f32,
    pub tz: f32,
}

impl Inverse for Scale {
    type Neg = Self;
    fn generate_inverse(&self) -> Self::Neg {
        Scale {
            tx: 1.0 / self.tx,
            ty: 1.0 / self.ty,
            tz: 1.0 / self.tz,
        }
    }
}
impl Mat for Scale {
    fn generate(&self) -> Mat4 {
        Mat4::from_cols_array(&[
            self.tx, 0., 0., 0., 0., self.ty, 0., 0., 0., 0., self.tz, 0., 0., 0., 0., 1.0,
        ])
    }
}

#[derive(Copy, Clone, Debug)]
pub struct XRot {
    pub angle_rad: f32,
}
impl Inverse for XRot {
    type Neg = Self;
    fn generate_inverse(&self) -> Self::Neg {
        XRot {
            angle_rad: -self.angle_rad,
        }
    }
}
impl Mat for XRot {
    fn generate(&self) -> Mat4 {
        let c = self.angle_rad.cos();
        let s = self.angle_rad.sin();

        Mat4::from_cols_array(&[1., 0., 0., 0., 0., c, s, 0., 0., -s, c, 0., 0., 0., 0., 1.])
    }
}

#[derive(Copy, Clone, Debug)]
pub struct YRot {
    pub angle_rad: f32,
}
impl Inverse for YRot {
    type Neg = Self;
    fn generate_inverse(&self) -> Self::Neg {
        YRot {
            angle_rad: -self.angle_rad,
        }
    }
}
impl Mat for YRot {
    fn generate(&self) -> Mat4 {
        let c = self.angle_rad.cos();
        let s = self.angle_rad.sin();

        Mat4::from_cols_array(&[c, 0., -s, 0., 0., 1., 0., 0., s, 0., c, 0., 0., 0., 0., 1.])
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ZRot {
    pub angle_rad: f32,
}
impl Inverse for ZRot {
    type Neg = Self;
    fn generate_inverse(&self) -> Self::Neg {
        ZRot {
            angle_rad: -self.angle_rad,
        }
    }
}
impl Mat for ZRot {
    fn generate(&self) -> Mat4 {
        let c = self.angle_rad.cos();
        let s = self.angle_rad.sin();

        Mat4::from_cols_array(&[c, s, 0., 0., -s, c, 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.])
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

#[derive(Clone, Debug)]
pub struct Translation {
    tx: f32,
    ty: f32,
    tz: f32,
}

impl Inverse for Translation {
    type Neg = Self;
    fn generate_inverse(&self) -> Self::Neg {
        Translation {
            tx: -self.tx,
            ty: -self.ty,
            tz: -self.tz,
        }
    }
}
impl Mat for Translation {
    fn generate(&self) -> Mat4 {
        let tx = self.tx;
        let ty = self.ty;
        let tz = self.tz;
        Mat4::from_cols_array(&[
            1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., tx, ty, tz, 1.,
        ])
    }
}

///
/// Chain together a list of elements
///
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
