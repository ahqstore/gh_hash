# gh_hash

Generate AHQ Store Username Hashes

```rs
use ahqstore_gh_hash::compute;

pub fn main() {
  let hash = compute("my username");

  println!("{hash}");
}
```
