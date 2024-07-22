use rquickjs::{embed, loader::Bundle, Context, Ctx, Module, Object, Runtime};
use std::cell::RefCell;

static BUNDLE: Bundle = embed! {
    "katex": "target/katex-js-release/katex.mjs",
};

fn init_ctx() -> Context {
    let runtime = Runtime::new().unwrap();
    let context = Context::full(&runtime).unwrap();
    runtime.set_loader(BUNDLE, BUNDLE);
    context
}

thread_local! {
    static CONTEXT: RefCell<Context> = RefCell::new(init_ctx());
}

pub fn get_version() -> String {
    CONTEXT.with_borrow(|context| {
        context.with(|context: Ctx| -> String {
            Module::import(&context, "katex")
                .unwrap()
                .finish::<Object>()
                .unwrap()
                .get::<_, Object>("default")
                .unwrap()
                .get::<_, String>("version")
                .unwrap()
        })
    })
}

#[test]
fn test() {
    assert_eq!(get_version(), "0.16.11");
}
