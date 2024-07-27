pub mod render_options;

pub use include_dir;

use include_dir::{include_dir as id_include_dir, Dir as IDDir};
use render_options::RenderOptions;
use rquickjs::{embed, loader::Bundle, Context, Ctx, Function, Module, Object, Runtime};
use std::cell::RefCell;

static BUNDLE: Bundle = embed! {
    "katex": "katex-js-release/katex.mjs",
};

pub static FONTS_DIR: IDDir = id_include_dir!("$CARGO_MANIFEST_DIR/katex-js-release/fonts");
pub static KATEX_MIN_CSS_FILE: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/katex-js-release/katex.min.css"
));

pub(crate) type QJSResult<T> = rquickjs::Result<T>;

fn init_ctx() -> Context {
    let runtime = Runtime::new().unwrap();
    let context = Context::full(&runtime).unwrap();
    runtime.set_loader(BUNDLE, BUNDLE);
    context.with(|context| {
        let katex_module = Module::import(&context, "katex")
            .unwrap()
            .finish::<Object>()
            .unwrap();
        context.globals().set("katex", katex_module).unwrap();
    });
    context
}

thread_local! {
    static CONTEXT: RefCell<Context> = RefCell::new(init_ctx());
}

fn with_katex<R>(f: impl FnOnce(Object) -> QJSResult<R>) -> QJSResult<R> {
    CONTEXT.with_borrow(|context| {
        context.with(|context: Ctx| {
            context
                .globals()
                .get::<_, Object>("katex")?
                .get::<_, Object>("default")
                .and_then(f)
        })
    })
}

/// Get version of underlying KaTeX JS library
pub fn get_version() -> String {
    with_katex(|katex| katex.get("version")).unwrap()
}

/// Render LaTeX as HTML (no error handling yet)
pub fn render_to_string(latex: String) -> String {
    with_katex(|katex| {
        katex
            .get::<_, Function>("renderToString")
            .expect("missing 'katex.renderToString'")
            .call((latex, RenderOptions::default()))
            .expect("no exceptions should have been thrown")
    })
    .unwrap()
}

#[test]
fn test_version() {
    use std::{env, fs, path::PathBuf};

    let release_file = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("katex-release.txt");
    let version_name = fs::read_to_string(release_file).unwrap();
    let version_name = version_name.trim();
    assert_eq!(get_version(), version_name[1..]);
}

#[test]
fn test_render_to_string() {
    // Note: Make sure to update this when incrementing 'katex-release.txt'
    assert_eq!(render_to_string("2 + 2 = 4".to_string()), "<span class=\"katex\"><span class=\"katex-mathml\"><math xmlns=\"http://www.w3.org/1998/Math/MathML\"><semantics><mrow><mn>2</mn><mo>+</mo><mn>2</mn><mo>=</mo><mn>4</mn></mrow><annotation encoding=\"application/x-tex\">2 + 2 = 4</annotation></semantics></math></span><span class=\"katex-html\" aria-hidden=\"true\"><span class=\"base\"><span class=\"strut\" style=\"height:0.7278em;vertical-align:-0.0833em;\"></span><span class=\"mord\">2</span><span class=\"mspace\" style=\"margin-right:0.2222em;\"></span><span class=\"mbin\">+</span><span class=\"mspace\" style=\"margin-right:0.2222em;\"></span></span><span class=\"base\"><span class=\"strut\" style=\"height:0.6444em;\"></span><span class=\"mord\">2</span><span class=\"mspace\" style=\"margin-right:0.2778em;\"></span><span class=\"mrel\">=</span><span class=\"mspace\" style=\"margin-right:0.2778em;\"></span></span><span class=\"base\"><span class=\"strut\" style=\"height:0.6444em;\"></span><span class=\"mord\">4</span></span></span></span>");
}
