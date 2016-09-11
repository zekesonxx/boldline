//! boldline is a small utility binary and library for generating bold lines in repeated rows of text.
//!
//! # Some Examples:
//!
//! `boldline("boldline".as_string(), Marking::HTMLBold, Pattern::Left);`
//!
//! <b>b</b>oldline<br/>
//! b<b>o</b>ldline<br/>
//! bo<b>l</b>dline<br/>
//! bol<b>d</b>line<br/>
//! bold<b>l</b>ine<br/>
//! boldl<b>i</b>ne<br/>
//! boldli<b>n</b>e<br/>
//! boldlin<b>e</b>
//!
//! `boldline("boldline".as_string(), Marking::HTMLBold, Pattern::Right);`
//!
//! boldlin<b>e</b><br/>
//! boldli<b>n</b>e<br/>
//! boldl<b>i</b>ne<br/>
//! bold<b>l</b>ine<br/>
//! bol<b>d</b>line<br/>
//! bo<b>l</b>dline<br/>
//! b<b>o</b>ldline<br/>
//! <b>b</b>oldline
//!
//! `boldline("boldline".as_string(), Marking::HTMLBold, Pattern::Cross);`
//!
//! <b>b</b>oldlin<b>e</b><br/>
//! b<b>o</b>ldli<b>n</b>e<br/>
//! bo<b>l</b>dl<b>i</b>ne<br/>
//! bol<b>dl</b>ine<br/>
//! bo<b>l</b>dl<b>i</b>ne<br/>
//! b<b>o</b>ldli<b>n</b>e<br/>
//! <b>b</b>oldlin<b>e</b>
//!

#![allow(dead_code)]

static DONT_BOLD: [char; 3] = [' ', '\'', '.'];

fn should_bold(c: char) -> bool {
    for check in DONT_BOLD.iter() {
        if c == *check {
            return false;
        }
    }
    true
}

/// The markup to use
///
/// You can use Marking::custom() to create a custom type if none of these strike your fancy.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Marking {
    /// ANSI Terminal Escapes
    ANSIBold,
    /// Markdown `(**c**)`
    MarkdownBold,
    /// BBCode `[b]c[/b]`
    BBCodeBold,
    /// HTML `<b>c</b>`
    HTMLBold,
    /// A custom type. See `Marking::custom()` for more info
    Custom(bool, String, String)
}

impl Marking {
    /// Creates a custom type to pass to boldline().
    ///
    /// The arguments are dedupe, prefix, and suffix.
    ///
    /// Dedupe refers to whether or not it should automatically prevent suffixprefix
    /// when using the Pattern::Cross.
    ///
    /// dedupe true: `bol<b>dl</b>ine`; dedupe false: `bol<b>d</b><b>l</b>ine`.
    pub fn custom(dedupe: bool, prefix: String, suffix: String) -> Self {
        Marking::Custom(dedupe, prefix, suffix)
    }
    /// This is what boldline() uses to get usable information out of the Marking enum.
    fn vars(self) -> (bool, String, String) {
        use self::Marking::*;
        match self {
            ANSIBold => (false, "\x1B[1m".to_string(), "\x1B[0m".to_string()),
            MarkdownBold => (true, "**".to_string(), "**".to_string()),
            BBCodeBold => (true, "[b]".to_string(), "[/b]".to_string()),
            HTMLBold => (true, "<b>".to_string(), "</b>".to_string()),
            Custom(a, b, c) => (a, b, c)
        }
    }
}

/// The pattern to bold
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Pattern {
    // A line going left to right
    Left,
    // A line going right to left
    Right,
    // Left and Right combined
    Cross
}
impl Pattern {
    /// (leftwise, rightwise)
    fn vars(self) -> (bool, bool) {
        use self::Pattern::*;
        match self {
            Left => (true, false),
            Right => (false, true),
            Cross => (true, true)
        }
    }
}

/// Does the bold line generation
///
/// Returns a vector of lines (Strings), instead of joining them together automtically.
/// This way, you can fit it in whereever you want, with (hopefully) no post-processing necessary.
pub fn boldline(input: String, marking: Marking, pattern: Pattern) -> Vec<String> {
    // Var setup
    let (leftwise, rightwise) = pattern.vars();
    let both = leftwise && rightwise;
    let (dedupe, prefix, suffix) = marking.vars();
    let mut output: Vec<String> = vec![];


    let mut was_next_to_last_iter = false;
    // This loop does each line
    for left in 0..input.len() {
        let right = input.len()-left-1;


        // Check if it's a character we shouldn't bold (like a space)
        if (leftwise && !should_bold(input[left..left+1].as_bytes()[0] as char)) ||
           (rightwise && !should_bold(input[right..right+1].as_bytes()[0] as char)) {
            continue;
        }


        //Check if they're right next to eachother
        let mut next_to_eachother = false;
        if left+1 == right || right+1 == left {
            next_to_eachother = true;
        } else {
            was_next_to_last_iter = false;
        }

        //prevent a double line in the middle
        if next_to_eachother && was_next_to_last_iter && pattern == Pattern::Cross {
            continue;
        } else if next_to_eachother {
            was_next_to_last_iter = true;
        }


        let mut line = String::new();
        // This loop does each character in the line
        for j in 0..input.len() {
            let c = &input[j..j+1];
            let leftchar = leftwise && left == j;
            let rightchar = rightwise && right == j;
            if leftchar || rightchar {
                if both && next_to_eachother && dedupe {
                    if (left < right && rightchar) ||
                    (left > right && leftchar) {
                        //Right char
                        line.push_str(c);
                        line.push_str(suffix.as_str());
                    } else {
                        //Left char
                        line.push_str(prefix.as_str());
                        line.push_str(c);
                    }
                } else {
                    //Isn't next to the other character to bold
                    //(or it's not a cross)
                    line.push_str(prefix.as_str());
                    line.push_str(c);
                    line.push_str(suffix.as_str());
                }
            } else {
                // Doesn't need bolding
                line.push_str(c);
            }
        }
        output.push(line);
    }
    output
}
