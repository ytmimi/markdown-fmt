#![no_main]

use libfuzzer_sys::fuzz_target;
use markdown_fmt::rewrite_markdown;
use std::panic::catch_unwind;

fuzz_target!(|text: String| {
    // ascii control characters can mess with parsing the input
    // and can lead to some issues when formatting the code.
    // pulldown-cmark also uses this guard in it's fuzz testing code
    if text.bytes().any(|b| b.is_ascii_control() && b != b'\n') {
        return;
    }

    let Ok(first_pass) = catch_unwind(|| rewrite_markdown(&text).unwrap()) else {
        eprintln!("Firt Pass Failed on input: {text:?}");
        return
    };

    let Ok(second_pass) = catch_unwind(|| rewrite_markdown(&first_pass).unwrap()) else {
        eprintln!("Second Pass Failed on input: {first_pass:?}");
        return
    };

    if second_pass != first_pass {
        panic!(
            "Formatting not idempotent.\ninput:  {:?}\nfirst:  {:?}\nsecond: {:?}",
            text,
            first_pass,
            second_pass,
        )
    }
    assert_eq!(first_pass, second_pass)
});
