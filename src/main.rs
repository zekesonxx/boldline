#[macro_use] extern crate clap;

mod lib;
use lib::*;

fn parse_marking<'a>(input: &'a str) -> Option<Marking> {
    match input {
        "ansibold"     | "ansi" | "terminal" | "t"            => Some(Marking::ANSIBold),
        "htmlbold"     | "html" | "h"                         => Some(Marking::HTMLBold),
        "markdownbold" | "markdown" | "md" | "gfm" | "reddit" => Some(Marking::MarkdownBold),
        "bbcodebold"   | "bbcode" | "bb"                      => Some(Marking::BBCodeBold),
        _ => None
    }
}

fn parse_pattern<'a>(input: &'a str) -> Option<Pattern> {
    match input {
        "l" |       "left"  => Some(Pattern::Left),
        "r" |       "right" => Some(Pattern::Right),
        "x" | "c" | "cross" => Some(Pattern::Cross),
        _ => None
    }
}

fn main() {

    let valid_marking = |marking: String| {
        match parse_marking(marking.as_str()) {
            Some(_) => Ok(()),
            None => Err(String::from("Invalid --marking selection"))
        }
    };
    let valid_pattern = |pattern: String| {
        match parse_pattern(pattern.as_str()) {
            Some(_) => Ok(()),
            None => Err(String::from("Invalid --pattern selection"))
        }
    };

    let matches = clap_app!(boldline =>
        (version: crate_version!())
        (about: "Generates a bolded line in a line of repeated text")
        (@setting ArgRequiredElseHelp)
        (@arg marking: -m --marking +takes_value {valid_marking} "Sets the markup to bold with")
        (@arg pattern: -p --pattern +takes_value {valid_pattern} "Sets the pattern of line to draw")
        (@arg input: +required "The line to bold")
    ).get_matches();
    let input = matches.value_of("input").unwrap().to_string();
    let marking = parse_marking(matches.value_of("marking").unwrap_or("ansi")).unwrap();
    let pattern = parse_pattern(matches.value_of("pattern").unwrap_or("left")).unwrap();
    let joiner = match marking {
        Marking::HTMLBold => "<br/>\n",
        Marking::MarkdownBold => "  \n",
        _ => "\n"
    };
    let output = boldline(input, marking, pattern);
    println!("{}", output.join(joiner));
}
