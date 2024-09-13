use sha256::digest;

/// Computes the hash
/// 
/// This method has no magic ✨ involved
/// its simply a sha256 digest algorithm
/// 
/// # Example
/// ```rs
/// use sha256::digest;
/// 
/// pub fn main() {
///   let data = digest("my data");
///   println!("{data}");
/// }
/// ```
pub fn compute<T: AsRef<[u8]>>(data: T) -> String {
  digest(data.as_ref())
}