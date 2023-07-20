

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

### CI.yml
如果将 check-latest 参数设置为 false，则 actions/setup-node@v3 action 将不会检查 Node.js 的最新可用版本，而是直接安装指定版本的 Node.js。

默认情况下，如果未指定或将 check-latest 设置为 true，则 actions/setup-node@v3 action 将检查满足 node-version 参数中指定的版本范围的最新可用版本的 Node.js。如果存在更新版本，则安装更新版本而非指定版本。

例如，如果 node-version 设置为 14.x，并且 check-latest 设置为 true，则 actions/setup-node@v3 action 将检查 14.x 范围内的最新可用版本的 Node.js，并安装它，如果它比当前安装的版本更新。如果 check-latest 设置为 false，则它将安装最新可用的满足 14.x 范围的版本，而不管它是否比当前安装的版本更新。

总之，将 check-latest 设置为 false 可以确保始终安装特定版本的 Node.js，而不管新版本是否可用。这在某些情况下是有用的。


### Publish
```shell
npm version patch
git push --follow-tags
```