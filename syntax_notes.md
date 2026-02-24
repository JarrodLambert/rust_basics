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

### Traits: The "Capability" License
A Trait defines a set of methods that different types can share. 
* **The Rule:** To use a trait's methods, you must `use` the trait at the top of the file.
* **Abstraction:** Functions can accept `impl Trait` instead of a specific type, making code hardware-agnostic.
* **Analogy:** - **Trait:** A "Driver's License" (Requirement: steer, brake).
    - **Struct:** A "Tesla" or "Truck" (The specific data/hardware).
    - **Instance:** Your specific car in the driveway (`let mut my_car = ...`).
