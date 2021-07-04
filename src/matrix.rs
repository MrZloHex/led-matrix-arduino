pub struct Matrix {
    series: str
}

impl Matrix {
    pub fn new() -> Matrix {
        Matrix {
            series: "1588AS"
        }
    }

    pub fn get_series(&self) -> str {
        self.series
    }
}