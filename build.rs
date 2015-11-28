extern crate gcc;

fn main() {
    gcc::compile_library("libhoedown.a",
        &["hoedown/src/autolink.c",
          "hoedown/src/buffer.c",
          "hoedown/src/document.c",
          "hoedown/src/escape.c",
          "hoedown/src/html.c",
          "hoedown/src/html_blocks.c",
          "hoedown/src/html_smartypants.c",
          "hoedown/src/stack.c",
          "hoedown/src/version.c",
         ]
    );
}
