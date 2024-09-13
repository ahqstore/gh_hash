use sha256::digest;

pub fn compute<T: AsRef<[u8]>>(data: T) -> String {
  digest(data.as_ref())
}