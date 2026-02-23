### Rust Syntax: Paths & Formatting

#### The Path Separator (`::`)
Used to navigate the hierarchy of crates, modules, and types. It essentially means "look inside."
* **Crate/Module Access:** `std::fs` (The `fs` module **inside** the `std` crate).
* **Associated Functions:** `Path::new()` (The `new` function belonging to the `Path` struct).
* **Enum Variants:** `Result::Ok` (The `Ok` variant **inside** the `Result` enum).

#### Debug Formatting (`{:?}`)
A placeholder for macros (like `println!`) used to inspect complex types.
* **`{}` (Display):** Only for types with a clear "user-facing" string version (like `i32` or `String`).
* **`{:?}` (Debug):** For types without a default string version (like `Path`, `Vec`, or `Results`). It prints the internal structure for developers.
* **`{:#?}` (Pretty Debug):** Same as `{:?}` but adds indentation and newlines—useful for large structs.
