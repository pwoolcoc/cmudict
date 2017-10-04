#![feature(try_from)]

extern crate cmudict_core;
use std::collections::HashMap;
use std::convert::TryFrom;
use cmudict_core::Rule;

// provides array of rules from the cmudict-0.7b file
include!(concat!(env!("OUT_DIR"), "/lib.rs"));

fn fixup_key(key: &str) -> Result<String, String> {
    match (key.starts_with(char::is_alphabetic),
           key.starts_with(char::is_numeric),
           key.ends_with(')'))
    {
        (x, y, true) if x || y => {
        let parts = key.split('(').next();
            match parts {
                Some(k) => Ok(k.to_string()),
                None => Err(format!("Error converting key? {}", key))
            }
        },
        (false, false, _) => {
            let chars: Vec<_> = key.chars()
                                   .take_while(|c| !c.is_alphabetic())
                                   .map(|c| c.to_string())
                                   .collect();
            let k = chars.join("");
            Ok(k)
        },
        _ => Ok(key.to_string())
    }
}

pub fn build_map(limit: Option<usize>) -> Result<HashMap<String, Vec<Rule>>, cmudict_core::Error> {
    let limit = limit.unwrap_or(100000);
    let mut map = HashMap::new();
    for rule_str in CMUDICT.iter().take(limit) {
        let rule = Rule::try_from(rule_str)?;
        let key = fixup_key(rule.label()).unwrap();
        println!("{}: {:?}", key, rule);
    }
    Ok(map)
}

