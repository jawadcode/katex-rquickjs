use rquickjs::{
    loader::{BuiltinLoader, BuiltinResolver},
    Context, Ctx, Function, Module, Object, Runtime,
};
use std::cell::RefCell;

const SCRIPT_MODULE: &'static str = "export const hi = name => `Hi ${name}!`;";

fn init_ctx() -> Context {
    let resolver = BuiltinResolver::default().with_module("hi");
    let loader = BuiltinLoader::default().with_module("hi", SCRIPT_MODULE);

    let runtime = Runtime::new().unwrap();
    let context = Context::full(&runtime).unwrap();

    runtime.set_loader(resolver, loader);

    context
}

thread_local! {
    static CONTEXT: RefCell<Context> = RefCell::new(init_ctx());
}

pub fn hi(name: String) -> String {
    CONTEXT.with_borrow(|context| {
        context.with(|context: Ctx| -> String {
            Module::import(&context, "hi")
                .unwrap()
                .finish::<Object>()
                .unwrap()
                .get::<_, Function>("hi")
                .unwrap()
                .call::<_, String>((name,))
                .unwrap()
        })
    })
}

#[test]
fn test() {
    assert_eq!(hi("Jawad".to_string()), "Hi Jawad!");
}
