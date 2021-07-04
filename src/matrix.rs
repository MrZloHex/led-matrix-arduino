use embedded_hal::digital::v2::{OutputPin, ToggleableOutputPin};

pub struct Matrix<'a, T: OutputPin + ToggleableOutputPin> {
    series: &'a str,
    // PINOUT
    row_a: &'a mut T
}

impl<T: OutputPin + ToggleableOutputPin> Matrix<'_, T> {
    pub fn new(serial: &'static str, r_a: &'static mut T) -> Matrix<'static, T> {
        Matrix {
            series: serial,
            row_a: r_a
        }
    }

    pub fn get_series(&mut self) -> &str {
        self.series
    }


    pub fn pin_toogle(&mut self) {
        match (*self.row_a).toggle() {
            Ok(_) => (),
            Err(_) => ()
        }
    }

}