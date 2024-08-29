# straico -p "Can I call lint a rust script programmatically? Is there a crate for that?"
Yes, you can programmatically lint a Rust script, and there are crates available to help you do this. The most common and official way to lint Rust code is by using Clippy, which is a collection of lints to catch common mistakes and improve your Rust code.

Here are a few options for programmatically linting Rust code:

1. Use the `rust-clippy` crate:
   The `rust-clippy` crate provides programmatic access to Clippy's lints. However, it's not typically used directly in applications, as it's more suited for integration into build tools and IDEs.

2. Use the `cargo_metadata` crate with Clippy:
   You can use the `cargo_metadata` crate to run Clippy as part of a Cargo command and parse its output. Here's a basic example:

   ```rust
   use cargo_metadata::{Message, MetadataCommand};
   use std::process::Command;

   fn main() {
       let metadata = MetadataCommand::new().exec().unwrap();
       let package_id = &metadata.workspace_packages[0];

       let output = Command::new("cargo")
           .args(&["clippy", "--message-format=json"])
           .output()
           .expect("Failed to execute clippy");

       for message in Message::parse_stream(output.stdout.as_slice()) {
           if let Message::CompilerMessage(msg) = message.unwrap() {
               println!("{:?}", msg);
           }
       }
   }
   ```

3. Use the `rustc-lint` crate:
   This crate provides a way to run the Rust compiler's built-in lints programmatically. It doesn't include Clippy lints, but it can be useful for basic linting:

   ```rust
   use rustc_lint::{LintStore, Level};
   use rustc_session::config::ErrorOutputType;
   use rustc_driver::{RunCompiler, Compilation};
   use rustc_interface::Config;

   fn main() {
       let args: Vec<String> = std::env::args().collect();
       let mut config = Config::default();
       config.opts.error_format = ErrorOutputType::JSON;

       RunCompiler::new(&args, &mut config).run(|compiler| {
           compiler.enter(|queries| {
               queries.global_ctxt().unwrap().enter(|tcx| {
                   let mut lint_store = LintStore::new();
                   lint_store.register_lints(&rustc_lint::DEFAULT_LINTS);

                   tcx.lint_level_at_node(&lint_store);
               })
           });
           Ok(())
       }).expect("Compilation failed");
   }
   ```

4. Use a system command to run Clippy:
   If you don't need fine-grained control over the linting process, you can simply run Clippy as a system command and capture its output:

   ```rust
   use std::process::Command;

   fn main() {
       let output = Command::new("cargo")
           .args(&["clippy", "--message-format=json"])
           .output()
           .expect("Failed to execute clippy");

       println!("Clippy output: {}", String::from_utf8_lossy(&output.stdout));
   }
   ```

Each of these methods has its own trade-offs in terms of complexity, flexibility, and dependencies. Choose the one that best fits your specific use case.

Remember to add the necessary dependencies to your `Cargo.toml` file for whichever method you choose.
