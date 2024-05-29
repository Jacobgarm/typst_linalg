struct Vector {
    entries: Vec<f64>,
}

impl Vector {
    pub fn length(&self) -> f64 {
        self.entries.iter().map(|x| x * x).sum::<f64>().sqrt()
    }
}
