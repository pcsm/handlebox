Handlebox
==================================================================

Handlebox is a simple map-like collection that reuses unused keys. Right now it's hard-coded to use `u32` keys.

To install, add this line to your Cargo.toml:

```toml
[dependencies]
handlebox = "0.2.0"
```

Note that Handlebox has not yet reached version 1.0, so the API may change drastically between releases.

## Example

```rs
use handlebox::*;

// Creating
let mut c = HandleBox::new();

// Adding values
let h1 = c.add(888);

// Accessing values
assert_eq!(c.get(&h1).unwrap(), &888);

// Removing values
c.remove(&h1);

// You can access the internal `BTreeMap<u32, V>` with the `.internal_map()` method
assert_eq!(c.internal_map().values().len(), 0);
```