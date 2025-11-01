# Cargo cheetsheet

### 🔧 Main Functions of Cargo

1. **Creating a new project**

   ```bash
   cargo new my_project
   ```

   This command creates a standard Rust project structure:

   ```
   my_project/
   ├── Cargo.toml       # project configuration
   └── src/
       └── main.rs      # source code
   ```

2. **Building the project**

   ```bash
   cargo build
   ```

   Compiles the project and creates an executable in `target/debug/`.

3. **Running the program**

   ```bash
   cargo run
   ```

   Automatically builds (if needed) and runs the binary.
   So instead of two commands (`rustc main.rs` and `./main`), you just need one.

4. **Running tests**

   ```bash
   cargo test
   ```

   Runs all the tests in the project.

5. **Managing dependencies**
   In the `[dependencies]` section of `Cargo.toml`, you can specify external libraries (called *crates*):

   ```toml
   [dependencies]
   rand = "0.8"
   ```

   After that, Cargo will automatically download, compile, and link the required crates from [crates.io](https://crates.io).

6. **Building for release**

   ```bash
   cargo build --release
   ```

   Compiles your code with optimizations and puts the binary into `target/release/`.

7. **Publishing a crate**

   ```bash
   cargo publish
   ```

   Publishes your crate to [crates.io](https://crates.io) so others can use it.

---

### 📁 Cargo Project Structure

```
project/
├── Cargo.toml         # package configuration (name, version, dependencies)
├── Cargo.lock         # locks exact dependency versions
└── src/
    └── main.rs        # main source file (or lib.rs for libraries)
```

---

### 📦 What `cargo init` Does

The command:

```bash
cargo init
```

Turns an existing folder with a Rust file (like `main.rs`) into a full Cargo project by:

* creating a `Cargo.toml` with default configuration;
* creating a `src` directory if it doesn’t exist;
* moving your code into that `src` directory.
