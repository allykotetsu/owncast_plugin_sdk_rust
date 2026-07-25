# owncast_plugin_sdk_rust
A Rust crate for the Owncast Plugin SDK

# Introduction
owncast_plugin_sdk_rust is an SDK for creating Owncast plugins. [Owncast](https://owncast.online/) is a software for self-hosting livestreams, and it allows users to load plugins that export specific functions through Extism. To read more about how Owncast plugins work, please visit [owncast/plugin-sdk](https://github.com/owncast/plugin-sdk).

# WIP
This crate is a very early work in progress. When it's more complete and functional I will publish it to crates.io, but for now it will just stay here. 

The development of this crate also depends on how the plugin system for Owncast evolves. It's also in very early stages and is subject to change, so as it changes this repo will likely change as well, perhaps very drastically.

# Usage

## Creating your plugin
To start, create a project for a Rust binary in your IDE of choice. If you don't want to use an IDE and would prefer to create your project from the command line, please refer to https://doc.rust-lang.org/stable/book/ch01-00-getting-started.html.

Your `Cargo.toml` file must include these two dependencies: `extism-pdk` and `owncast_plugin_sdk_rust`. For the moment `owncast_plugin_sdk_rust` is not available on crates.io, so in order to use it as a dependency you must clone this git repo and refer to the repo by path in `Cargo.toml`.

Open up your `main.rs` file and start by writing the following:
```rust
use owncast_plugin_sdk_rust::prelude::*;

define_plugin!(|mut plugin_builder| {
    Ok(plugin_builder)
});

fn main() {}
```
This is the most basic a plugin can be. It doesn't add any functionality, but it can be registered by Owncast. Next let's break down what this all means.

## Coding
To create a plugin, use the `define_plugin!` macro. It takes a function pointer as a parameter. The function pointer returns a `FnResult<PluginBuilder>`, and it takes a `PluginBuilder` as a parameter.

You must use `define_plugin!` outside of function scope, as the macro expands to create a `const PLUGIN` variable, and global functions. The functions are Extism exports that return data through the `PLUGIN` variable.

To add functionality to your plugin, call the `PluginBuilder`'s functions. The following example is a simple chat echo bot, stripped down to just the macro and its contents.
```rust
define_plugin!(|mut plugin_builder| {
    // When a message is sent in the Owncast chat, run some code.
    plugin_builder.on_chat_message(|ChatMessage { body, .. }| {
        // Send a chat message.
        owncast_send_chat(&format!("echo {body}"));
    });
    // If there have been no errors thus far, then return an Ok.
    Ok(plugin_builder)
});
```
To learn how to add more functionality to your plugin, please read the [Wiki](https://github.com/allykotetsu/owncast_plugin_sdk_rust/wiki). There are also examples (WIP).

## Building
When your plugin is ready to build, do the following:
1. Run `cargo build --target wasm32-unknown-unknown --release` in the terminal.
2. Move it into a directory that contains a `plugin.manifest.json` file.
3. Rename the built `*.wasm` file to `plugin.wasm`.
4. Run `zip -q "<slug>.ocpkg" plugin.wasm plugin.manifest.json`, where `<slug>` is the unique identifier for your plugin.

# Using the plugin.
Now that your plugin is built, let's load it into Owncast.
1. Take your exported `.ocpkg` file and move it into your owncast data directory under plugins.
2. Go to the Plugins page on Owncast. If you don't see your plugin, click "Refresh".
3. Once your plugin shows up, click the switch to enable it.
4. The plugin should now be loaded! If there was an error doing so, it will be displayed. If your plugin needs to have settings configured, then click "Configuration" and do so.

# Conclusion
That's how you make an Owncast plugin using owncast_plugin_sdk_rust! As mentioned before, this is still an early WIP, so if you find any bugs or have any feature requests then send then submit them through the [Issues](https://github.com/allykotetsu/owncast_plugin_sdk_rust/issues) page.