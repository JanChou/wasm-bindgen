use super::project;

#[test]
fn works() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section, wasm_import_module)]

            extern crate wasm_bindgen;

            use std::cell::Cell;
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen(module = "./test")]
            extern {
                fn call(a: &Fn());
                fn thread(a: &Fn(u32) -> u32) -> u32;
            }

            #[wasm_bindgen]
            pub fn run() {
                let a = Cell::new(false);
                call(&|| a.set(true));
                assert!(a.get());

                assert_eq!(thread(&|a| a + 1), 3);
            }
        "#)
        .file("test.ts", r#"
            import { run } from "./out";

            export function call(a: any) {
                a();
            }

            export function thread(a: any) {
                return a(2);
            }

            export function test() {
                run();
            }
        "#)
        .test();
}

#[test]
fn cannot_reuse() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section, wasm_import_module)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen(module = "./test")]
            extern {
                fn call(a: &Fn());
                #[wasm_bindgen(catch)]
                fn call_again() -> Result<(), JsValue>;
            }

            #[wasm_bindgen]
            pub fn run() {
                call(&|| {});
                assert!(call_again().is_err());
            }
        "#)
        .file("test.ts", r#"
            import { run } from "./out";

            let CACHE: any = null;

            export function call(a: any) {
                CACHE = a;
            }

            export function call_again() {
                CACHE();
            }

            export function test() {
                run();
            }
        "#)
        .test();
}

#[test]
fn long_lived() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section, wasm_import_module)]

            extern crate wasm_bindgen;

            use std::cell::Cell;
            use std::rc::Rc;
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen(module = "./test")]
            extern {
                fn call1(a: &Closure<Fn()>);
                fn call2(a: &Closure<FnMut(u32) -> u32>) -> u32;
            }

            #[wasm_bindgen]
            pub fn run() {
                let hit = Rc::new(Cell::new(false));
                let hit2 = hit.clone();
                let a = Closure::new(move || hit2.set(true));
                assert!(!hit.get());
                call1(&a);
                assert!(hit.get());

                let hit = Rc::new(Cell::new(false));
                {
                    let hit = hit.clone();
                    let a = Closure::new(move |x| {
                        hit.set(true);
                        x + 3
                    });
                    assert_eq!(call2(&a), 5);
                }
                assert!(hit.get());
            }
        "#)
        .file("test.ts", r#"
            import { run } from "./out";

            export function call1(a: any) {
                a();
            }

            export function call2(a: any) {
                return a(2);
            }

            export function test() {
                run();
            }
        "#)
        .test();
}

#[test]
fn many_arity() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section, wasm_import_module)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen(module = "./test")]
            extern {
                fn call1(a: &Closure<Fn()>);
                fn call2(a: &Closure<Fn(u32)>);
                fn call3(a: &Closure<Fn(u32, u32)>);
                fn call4(a: &Closure<Fn(u32, u32, u32)>);
                fn call5(a: &Closure<Fn(u32, u32, u32, u32)>);
                fn call6(a: &Closure<Fn(u32, u32, u32, u32, u32)>);
                fn call7(a: &Closure<Fn(u32, u32, u32, u32, u32, u32)>);
                fn call8(a: &Closure<Fn(u32, u32, u32, u32, u32, u32, u32)>);

                #[wasm_bindgen(js_name = call1)]
                fn stack1(a: &Fn());
                #[wasm_bindgen(js_name = call2)]
                fn stack2(a: &Fn(u32));
                #[wasm_bindgen(js_name = call3)]
                fn stack3(a: &Fn(u32, u32));
                #[wasm_bindgen(js_name = call4)]
                fn stack4(a: &Fn(u32, u32, u32));
                #[wasm_bindgen(js_name = call5)]
                fn stack5(a: &Fn(u32, u32, u32, u32));
                #[wasm_bindgen(js_name = call6)]
                fn stack6(a: &Fn(u32, u32, u32, u32, u32));
                #[wasm_bindgen(js_name = call7)]
                fn stack7(a: &Fn(u32, u32, u32, u32, u32, u32));
                #[wasm_bindgen(js_name = call8)]
                fn stack8(a: &Fn(u32, u32, u32, u32, u32, u32, u32));
            }

            #[wasm_bindgen]
            pub fn run() {
                call1(&Closure::new(|| {}));
                call2(&Closure::new(|a| assert_eq!(a, 1)));
                call3(&Closure::new(|a, b| assert_eq!((a, b), (1, 2))));
                call4(&Closure::new(|a, b, c| assert_eq!((a, b, c), (1, 2, 3))));
                call5(&Closure::new(|a, b, c, d| {
                    assert_eq!((a, b, c, d), (1, 2, 3, 4))
                }));
                call6(&Closure::new(|a, b, c, d, e| {
                    assert_eq!((a, b, c, d, e), (1, 2, 3, 4, 5))
                }));
                call7(&Closure::new(|a, b, c, d, e, f| {
                    assert_eq!((a, b, c, d, e, f), (1, 2, 3, 4, 5, 6))
                }));
                call8(&Closure::new(|a, b, c, d, e, f, g| {
                    assert_eq!((a, b, c, d, e, f, g), (1, 2, 3, 4, 5, 6, 7))
                }));

                stack1(&(|| {}));
                stack2(&(|a| assert_eq!(a, 1)));
                stack3(&(|a, b| assert_eq!((a, b), (1, 2))));
                stack4(&(|a, b, c| assert_eq!((a, b, c), (1, 2, 3))));
                stack5(&(|a, b, c, d| {
                    assert_eq!((a, b, c, d), (1, 2, 3, 4))
                }));
                stack6(&(|a, b, c, d, e| {
                    assert_eq!((a, b, c, d, e), (1, 2, 3, 4, 5))
                }));
                stack7(&(|a, b, c, d, e, f| {
                    assert_eq!((a, b, c, d, e, f), (1, 2, 3, 4, 5, 6))
                }));
                stack8(&(|a, b, c, d, e, f, g| {
                    assert_eq!((a, b, c, d, e, f, g), (1, 2, 3, 4, 5, 6, 7))
                }));
            }
        "#)
        .file("test.ts", r#"
            import { run } from "./out";

            export function call1(a: any) { a() }
            export function call2(a: any) { a(1) }
            export function call3(a: any) { a(1, 2) }
            export function call4(a: any) { a(1, 2, 3) }
            export function call5(a: any) { a(1, 2, 3, 4) }
            export function call6(a: any) { a(1, 2, 3, 4, 5) }
            export function call7(a: any) { a(1, 2, 3, 4, 5, 6) }
            export function call8(a: any) { a(1, 2, 3, 4, 5, 6, 7) }

            export function test() {
                run();
            }
        "#)
        .test();
}

#[test]
fn long_lived_dropping() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section, wasm_import_module)]

            extern crate wasm_bindgen;

            use std::cell::Cell;
            use std::rc::Rc;
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen(module = "./test")]
            extern {
                fn cache(a: &Closure<Fn()>);
                #[wasm_bindgen(catch)]
                fn call() -> Result<(), JsValue>;
            }

            #[wasm_bindgen]
            pub fn run() {
                let hit = Rc::new(Cell::new(false));
                let hit2 = hit.clone();
                let a = Closure::new(move || hit2.set(true));
                cache(&a);
                assert!(!hit.get());
                assert!(call().is_ok());
                assert!(hit.get());
                drop(a);
                assert!(call().is_err());
            }
        "#)
        .file("test.ts", r#"
            import { run } from "./out";

            let CACHE: any = null;

            export function cache(a: any) { CACHE = a; }
            export function call() { CACHE() }

            export function test() {
                run();
            }
        "#)
        .test();
}

#[test]
fn long_fnmut_recursive() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section, wasm_import_module)]

            extern crate wasm_bindgen;

            use std::cell::Cell;
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen(module = "./test")]
            extern {
                fn cache(a: &Closure<FnMut()>);
                #[wasm_bindgen(catch)]
                fn call() -> Result<(), JsValue>;
            }

            #[wasm_bindgen]
            pub fn run() {
                let a = Closure::new(|| {
                    assert!(call().is_err());
                });
                cache(&a);
                assert!(call().is_ok());
            }
        "#)
        .file("test.ts", r#"
            import { run } from "./out";

            let CACHE: any = null;

            export function cache(a: any) { CACHE = a; }
            export function call() { CACHE() }

            export function test() {
                run();
            }
        "#)
        .test();
}
