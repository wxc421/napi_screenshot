

## build.rs
在 Rust 项目中，build.rs 是一个特殊的 Rust 文件，用于在构建过程中自定义构建行为。当您运行 cargo build 或 cargo run 命令时，build.rs 文件会在编译前运行，并生成 Rust 代码、C 代码、Makefile 或其他构建文件。

build.rs 文件通常用于执行以下操作：

生成 Rust 代码。例如，您可以使用 build.rs 文件生成类似于 constants 模块的代码，该模块包含在编译时生成的常量或配置信息。

生成 C 代码。例如，您可以使用 build.rs 文件生成 C 代码，该代码包含在 Rust 代码中使用的外部 C 库的绑定。

生成 Makefile 或其他构建文件。例如，您可以使用 build.rs 文件生成 Makefile，该文件包含在编译时使用的自定义构建规则


## Example
```node
yarn build
node example.mjs
```