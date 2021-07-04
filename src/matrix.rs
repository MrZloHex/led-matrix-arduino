pub struct Matrix<'a> {
    series: &'a str
}

impl Matrix<'_> {
    pub fn new(serial: &str) -> Matrix<'static> {
        Matrix {
            series: serial
        }
    }

    pub fn get_series(&self) -> &str {
        self.series
    }
}