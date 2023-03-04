use syntect::easy::HighlightLines;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{Color, ThemeSet, Style};
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};
use syntect::html::highlighted_html_for_string;

fn main() {
    // Load these once at the start of your program
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let syntax = ss.find_syntax_by_extension("rs").unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
    let file = "pub struct Wow { hi: u64 }\nfn blah() -> u64 {}\n";
    for line in LinesWithEndings::from(file) {
        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ss).unwrap();
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        print!("{}", escaped);
    }


    println!("");
    println!("");
    let filepath ="WHAT";

    // Thanks to: Google and Fuchsia for the following snippet (that I've updated):
    // https://fuchsia.googlesource.com/third_party/rust-crates/+/b49c02d3c94353e43c771e069ecce043ce2d355e/vendor/syntect/examples/synhtml.rs

    let style = "
        pre {
            font-size:13px;
            font-family: Consolas, \"Liberation Mono\", Menlo, Courier, monospace;
        }";
    println!("<head><title>{}</title><style>{}</style></head>", filepath, style);
    let theme = &ts.themes["base16-ocean.dark"];
    let c = theme.settings.background.unwrap_or(Color::WHITE);
    println!("<body style=\"background-color:#{:02x}{:02x}{:02x};\">\n", c.r, c.g, c.b);
    let html = highlighted_html_for_string(&file, &ss, &syntax, theme).unwrap();
    println!("{}", html);
    println!("</body>");
}
