
#[allow(dead_code)]
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

impl Convertable for i64 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_string().as_bytes().to_vec()
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        let res_str = std::str::from_utf8(bytes);
        match res_str {
            Ok(str) => {
                let res_int = str.parse::<i64>();
                match res_int {
                    Ok(int) => Ok(int),
                    Err(err) => Err(err.to_string()),
                }
            }
            Err(err) => Err(err.to_string()),
        }
    }
}

impl Convertable for usize {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_string().as_bytes().to_vec()
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        let res_str = std::str::from_utf8(bytes);
        match res_str {
            Ok(str) => {
                let res_index = str.parse::<usize>();
                match res_index {
                    Ok(index) => Ok(index),
                    Err(err) => Err(err.to_string()),
                }
            }
            Err(err) => Err(err.to_string()),
        }
    }
}
