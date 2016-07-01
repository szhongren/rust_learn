// rust provides a foreign function interface (FFI) to C libs
// foreign fns muts be declared inside an extern block annotated with a #[link]
// attrib containing the name of the foreign lib
use std::fmt;

// this extern block links to the libm library
#[link(name = "m")]
extern {
    // this is a foreign fn
    // that computes sqrt of a single precision complex number
    fn csqrtf(z: Complex) -> Complex;
}

fn main() {
    // z = -1 + 0i
    let z = Complex { re: -1., im: 0. };

    // calling a foreign fn is an unsafe operation
    let z_sqrt = unsafe {
        csqrtf(z)
    };

    println!("The square root of {:?} is {:?}", z, z_sqrt);
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Complex {
    re: f32,
    im: f32,
}

impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.im < 0. {
            write!(f, "{}-{}i", self.re, -self.im)
        } else {
            write!(f, "{}+{}i", self.re, self.im)
        }
    }
}