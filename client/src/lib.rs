//#![deny(warnings)]
use app::App;
use sauron::jss::jss;
use sauron::prelude::*;
use sauron::Window;
use ultron::editor;
use ultron::editor::Editor;
use ultron::nalgebra::Point2;

pub use ultron::nalgebra;
mod app;

pub const APP_CONTAINER: &str = "app_container";
pub const APP_TITLE: &str = "Madnotes";
pub const APP_JS_FILE: &str = "./pkg/client.js";
pub const APP_WASM_FILE: &str = "./pkg/client_bg.wasm";
pub const FAVICON_ICO: &str = "favicon.ico";

#[derive(Clone)]
pub struct Settings {
    pub app_container: &'static str,
    pub app_title: String,
    pub app_js_file: &'static str,
    pub app_wasm_file: &'static str,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            app_container: APP_CONTAINER,
            app_title: APP_TITLE.to_string(),
            app_js_file: APP_JS_FILE,
            app_wasm_file: APP_WASM_FILE,
        }
    }
}

#[cfg(feature = "external-invoke")]
#[wasm_bindgen]
extern "C" {
    fn invoke(arg: &str);
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();
    let app_container = sauron::document()
        .get_element_by_id(APP_CONTAINER)
        .expect("must have the #app_container in the page::index");

    let content = MARKDOWN_EXAMPLE;
    //let content = ""; // it would crash when using the desktop-app when content is preloaded with long text
    Program::replace_mount(App::with_content(content), &app_container);
}

const MARKDOWN_EXAMPLE: &str = r#"
Madnotes
============

Madnotes is and opensource application that allows you to create and share documents that can contain
live code, equations, data visualization and narrative text.
The files are uploaded in ipfs to preserve the linked components and data being referenced.

## Features
- Uses `ipfs` to upload and reference files
- Data visualization and charting
- Primarily use csv for data processing and storage
- Supports running SQL against a csv data

## Security
- Sharing of notes is safe as the scripts is run in a sandbox environment using wasm.
- Referenced files are coming from ipfs node and therefore the runtime is not allowed to access local files of the user.



You can write madnotes using markdown.

> Block quotes are
> written like so.


```info
You can create information text like this
```

```note
Note that madnotes can be extended with plugins
```


```warning
There is also a warning admonition.
```

You can write list as you would with markdown
Here's a numbered list:
 1. first item
 2. second item
 3. third item

```note
Note that madnotes, overloaded the code fence tag a lot
```
You can write scripts using
```rune
pub fn main(number) {
    add(number) + 10
}
```

Currently we only support 1 scripting language which is Rune.

~~~
define foobar() {
    print "Welcome to flavor country!";
}
~~~

(which makes copying & pasting easier). You can optionally mark the
delimited block for Pandoc to syntax highlight it:

~~~python
import time
# Quick, count to ten!
for i in range(10):
    # (but not *too* quick)
    time.sleep(0.5)
    print(i)
~~~



### An h3 header ###

Now a nested list:

 1. First, get these ingredients:

      * carrots
      * celery
      * lentils

 2. Boil some water.

 3. Dump everything in the pot and follow
    this algorithm:

```
    find wooden spoon
    uncover pot
    stir
    cover pot
    balance wooden spoon precariously on pot handle
    wait 10 minutes
    goto first step (or shut off burner when done)

    Do not bump wooden spoon or it will fall.
```

Notice again how text always lines up on 4-space indents (including
that last line which continues item 3 above).

Here's a link to [a website](http://foo.bar), to a [local
doc](local-doc.html), and to a [section heading in the current
doc](#an-h2-header). Here's a footnote [^1].

[^1]: Some footnote text.

Tables can look like this:

Name           Size  Material      Color
------------- -----  ------------  ------------
All Business      9  leather       brown
Roundabout       10  hemp canvas   natural
Cinderella       11  glass         transparent

Table: Shoes sizes, materials, and colors.

(The above is the caption for the table.) Pandoc also supports
multi-line tables:

--------  -----------------------
Keyword   Text
--------  -----------------------
red       Sunsets, apples, and
          other red or reddish
          things.

green     Leaves, grass, frogs
          and other things it's
          not easy being.
--------  -----------------------

A horizontal rule follows.

***

Here's a definition list:

apples
  : Good for making applesauce.

oranges
  : Citrus!

tomatoes
  : There's no "e" in tomatoe.

Again, text is indented 4 spaces. (Put a blank line between each
term and  its definition to spread things out more.)

Here's a "line block" (note how whitespace is honored):

| Line one
|   Line too
| Line tree

and images can be specified like so:

![example image](img/space.jpg "An exemplary image")

Inline math equation: $\omega = d\phi / dt$. Display
math should get its own line like so:

$$I = \int \rho R^{2} dV$$

And note that you can backslash-escape any punctuation characters
which you wish to be displayed literally, ex.: \`foo\`, \*bar\*, etc.
"#;
