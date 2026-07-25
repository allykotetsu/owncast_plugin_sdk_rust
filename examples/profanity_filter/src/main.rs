use owncast_plugin_sdk_rust::prelude::*;
use owncast_plugin_sdk_rust::json_objects::filter_result::FilterResult;
use regex::Regex;

// Construct a list of banned words.
const WORDLIST: &[&str] = &["damn", "hell", "crap"];

// Since constructing a Regex is an expensive operation, only do it once per word, and wrap it in a Lazy.
const REGEXES: LazyLock<Vec<(&str, Regex)>> = LazyLock::new(|| {
    // Go through each word and create a vector of tuples containing that word and the regex based off of it.
    WORDLIST.iter().map(|x| (*x, Regex::new(format!(r"\b{x}\b").as_str()).unwrap())).collect()
});

define_plugin!(|mut plugin_builder| {
    plugin_builder.filter_chat_message(None, |chat_message| {
        let mut body = chat_message.body.clone();
        let mut modified = false;

        // Iterate through the regexes and try to match the regex to the message content.
        for (word, re) in &*REGEXES {
            if re.is_match(&body) {
                body = re.replace_all(&body, "*".repeat(word.len())).to_string();
                modified = true;
            }
        }

        // If the message was modified, then return a FilterResult::Modify, else FilterResult::Pass.
        if modified {
            FilterResult::modify(&chat_message, body.as_str())
        } else {
            FilterResult::pass()
        }
    })?;

    Ok(plugin_builder)
});

fn main() {}