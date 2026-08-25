use crate::fft::bit_reverser::BitReverser;
use num_complex::Complex;
use rayon::prelude::*;


type Real = f64;

pub type ComplexValue = Complex<Real>;

pub trait FourierTransform {
    fn get_size(&self) -> u8;
    fn get_sign(&self) -> Real;
    fn get_coefficient(&self, n : usize) -> Real;
    fn get_w(&self, n: usize) -> ComplexValue {
        let pi = std::f64::consts::PI;
        let angle = self.get_sign() * 2. * pi / (n as Real);
        ComplexValue::new(angle.cos(), angle.sin())
    }
    fn transform(&self, input: &[ComplexValue]) -> Vec<ComplexValue> {

        let log_size = self.get_size();
        let reverser = BitReverser::new(log_size);
        let mut y = reverser.get_reverse(input);

        for i in 0..log_size {
            let n_2 = 0b01 << i;
            let n = n_2 << 1;
            let w_n = self.get_w(n);
            let parallel_iter : Vec<usize> = (0..y.len()).step_by(n)
              .map(|i| i..i + n_2).flatten().collect();
            let par_y = parallel_iter.par_iter().map(|&i| {
                let j = i + n_2;
                let w = w_n.powu(i as u32);
                let t = w * y[j];
                vec![(i, y[i] + t), (j, y[i] - t)]
            }).flatten().collect::<Vec<_>>();

            for (i, v) in par_y {
                y[i] = v;
            }
        }
        let coefficient = self.get_coefficient(y.len());
        y.par_iter().map(|&v| v * coefficient).collect()
    }
}

pub struct PolynomialTransform {
    size: u8,
}

impl PolynomialTransform {
    pub fn new(size: u8) -> Self {
        Self { size }
    }
}
impl FourierTransform for PolynomialTransform {
    fn get_size(&self) -> u8 {
        self.size
    }
    fn get_sign(&self) -> Real {1.0}
    fn get_coefficient(&self, n : usize) -> Real {1.0}
}

pub struct InverseTransform {
    size: u8,
}
impl InverseTransform {
    pub fn new(size: u8) -> Self {
        Self { size }
    }
}
impl FourierTransform for InverseTransform {
    fn get_size(&self) -> u8 {self.size}
    fn get_sign(&self) -> Real {-1.0}
    fn get_coefficient(&self, n : usize) -> Real {1.0 / (n as Real)}
}

mod test {
    use super::*;
    use crate::{scenario, given, when, then};

    scenario!(fourier_transform "test poly and inverse transform" {
        given!("a series complex values" {
            let input = vec![
                ComplexValue::new(1.0, 0.0), ComplexValue::new(2.0, 0.0),
                ComplexValue::new(1.5, 0.0), ComplexValue::new(2.5, 0.0),
                ComplexValue::new(3.0, 0.0), ComplexValue::new(4.0, 0.0),
                ComplexValue::new(5.5, 0.0), ComplexValue::new(7.0, 0.0)];
        });
        when!("do poly and inverse transform" {
            let polynomial_trans = PolynomialTransform::new(4);
            let intermediate = polynomial_trans.transform(&input);
            let inverse_trans = InverseTransform::new(4);
            let output = inverse_trans.transform(&intermediate);
        });
        then!("it should be back to original series" {
            for i in 0..input.len() {
                assert!(input[i].re - output[i].re < 0.001);
                assert!(input[i].im - output[i].im < 0.001);
            }
        });
    });
}
