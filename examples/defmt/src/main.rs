#![no_std]
#![no_main]

use defmt_rtt as _;
use kfilter::Kalman1M;
use nalgebra::{Matrix1, Matrix1x2, Matrix2, Matrix2x1, SMatrix};
use panic_probe as _;

use cortex_m_rt::entry;

#[entry]
unsafe fn main() -> ! {
    defmt::info!("Hello, world!");
    let m = kfilter::measurement::LinearMeasurement::new(
        Matrix1::new(1.0),
        Matrix1::new(1.0),
        Matrix1::new(1.0),
    );
    defmt::info!("Measurement created: {:?}", m);
    let q = Matrix2::new(1.0, 0.1, 0.0, 1.0);
    let k = Kalman1M::new_with_input(
        q,
        SMatrix::identity(),
        Matrix2x1::new(0.0, 1.0),
        Matrix1x2::new(1.0, 0.0),
        SMatrix::identity(),
        SMatrix::zeros(),
    );

    defmt::info!("Kalman filter created: {:?}", k);
    defmt::flush();
    #[allow(clippy::empty_loop)]
    loop {}
}
