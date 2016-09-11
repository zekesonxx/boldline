<b>b</b>oldline<br/>
b<b>o</b>ldline<br/>
bo<b>l</b>dline<br/>
bol<b>d</b>line<br/>
bold<b>l</b>ine<br/>
boldl<b>i</b>ne<br/>
boldli<b>n</b>e<br/>
boldlin<b>e</b>

Generates a bold line in a repeated line of text.

# Installation
`cargo install boldline`

Make sure you have `$HOME/.cargo/bin` in your `$PATH`.

# Usage
```
$ boldline
boldline 0.1.0
Generates a bolded line in a line of repeated text

USAGE:
    boldline [OPTIONS] <input>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -m, --marking <marking>    Sets the markup to bold with
    -p, --pattern <pattern>    Sets the pattern of line to draw

ARGS:
    <input>    The line to bold
```

# Some Examples:

`boldline("boldline".as_string(), Marking::HTMLBold, Pattern::Left);` / `boldline -p left boldline`

<b>b</b>oldline<br/>
b<b>o</b>ldline<br/>
bo<b>l</b>dline<br/>
bol<b>d</b>line<br/>
bold<b>l</b>ine<br/>
boldl<b>i</b>ne<br/>
boldli<b>n</b>e<br/>
boldlin<b>e</b>

`boldline("boldline".as_string(), Marking::HTMLBold, Pattern::Right);` / `boldline -p right boldline`

boldlin<b>e</b><br/>
boldli<b>n</b>e<br/>
boldl<b>i</b>ne<br/>
bold<b>l</b>ine<br/>
bol<b>d</b>line<br/>
bo<b>l</b>dline<br/>
b<b>o</b>ldline<br/>
<b>b</b>oldline

`boldline("boldline".as_string(), Marking::HTMLBold, Pattern::Cross);` / `boldline -p cross boldline`

<b>b</b>oldlin<b>e</b><br/>
b<b>o</b>ldli<b>n</b>e<br/>
bo<b>l</b>dl<b>i</b>ne<br/>
bol<b>dl</b>ine<br/>
bo<b>l</b>dl<b>i</b>ne<br/>
b<b>o</b>ldli<b>n</b>e<br/>
<b>b</b>oldlin<b>e</b>


# Legal
Licensed under the Apache 2.0 license.
