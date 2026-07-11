# owncast_plugin_sdk_rust
A Rust crate for the Owncast Plugin SDK

# Introduction
owncast_plugin_sdk_rust is an SDK for creating Owncast plugins. Owncast is a software for self-hosting livestreams, and it allows users to load plugins that export specific functions through WASM.

# WIP
This crate is a very early work in progress. When it's more complete and functional I will publish it to crates.io, but for now it will just stay here. 

The development of this crate also depends on how the plugin system for Owncast evolves. It's also in very early stages and is subject to change, so as it changes this repo will likely change as well, perhaps very drastically.

# Usage
To create a plugin, use the define_plugin! macro. It takes a closure as a parameter. The closure returns a Result<PluginBuilder, Box<dyn Error>>, and it takes a PluginBuilder<'static> as a parameter.

To add functionality to your plugin, call the PluginBuilder's functions. The following example is a simple chat echo bot. (Use statement are not shown.)

You must use define_plugin! outside of function scope, as the macro expands to create a const PLUGIN variable, and global functions. The functions are WASM exports that return data through the PLUGIN variable.
```rust
define_plugin!(|mut plugin_builder: PluginBuilder<'static>| -> Result<PluginBuilder, Box<dyn Error>> {
    // When a message is sent in the Owncast chat, echo it back.
    plugin_builder.on_chat_message(|ChatMessage { body, .. }| {
        owncast_send_chat(format!("echo ${body}").as_str());
    });
    
    // When a user types "!update", "!time", or "!livetime", tell them that the stream has been live for a while.
    plugin_builder.commands("!", vec![
        CommandBuilder::new("update", &|ctx: Ctx| {
            ctx.reply("we've been live a while!");
        })
        .with_aliases(&["time", "livetime"])
        .with_cooldown(1000)
    ])?;
    // Since PluginBuilder::commands can error, we use ? to propagate that error to the macro. All errors panic through an unwrap().
    
    // If there have been no errors thus far, then return an Ok.
    Ok(plugin_builder)
});
```