pub trait Convertable: Sized {
    fn to_bytes(&self) -> Vec<u8>;

    fn from_bytes(bytes: &[u8]) -> Result<Self, String>;
}

impl Convertable for f64 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_string().as_bytes().to_vec()
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        let res_str = std::str::from_utf8(bytes);
        match res_str {
            Ok(str) => {
                let res_float = str.parse::<f64>();
                match res_float {
                    Ok(float) => Ok(float),
                    Err(err) => Err(err.to_string()),
                }
            }
            Err(err) => Err(err.to_string()),
        }
    }
}
