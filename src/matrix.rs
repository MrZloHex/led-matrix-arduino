pub struct Matrix<'a> {
    series: &'a str
}

impl Matrix<'_> {
    pub fn new() -> Matrix<'static> {
        Matrix {
            series: "1588AS"
        }
    }

    pub fn get_series(&self) -> &str {
        self.series
    }
}