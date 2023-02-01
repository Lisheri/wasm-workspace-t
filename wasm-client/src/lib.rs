mod utils;

use wasm_bindgen::prelude::*;

use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlButtonElement;
use wasm_bindgen::JsCast;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
// ? C 表示使用 application binary interface 调用程序二进制接口
extern "C" {
    fn alert(s: &str);
    fn confirm(s: &str) -> bool;

    #[wasm_bindgen(js_namespace = console)]
    // 获取 console.info
    fn info(s: &str);
}

pub mod errors;
pub mod models;

use models::course::{Course, delete_course};

// ? #[wasm_bindgen(start)] 表示下面的函数是一个入口函数, 项目启动就会执行, 异步非异步皆可
#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    info("执行main");
    let window = web_sys::window().expect("no global window exists");
    let document = window.document().expect("no global document exists");

    // 获取 tbody 这个元素
    let left_tbody = document
        .get_element_by_id("left-tbody")
        .expect("left div not exists");

    let courses: Vec<Course> = models::course::get_courses_by_teacher(1).await.unwrap();
    for c in courses.iter() {
        let tr = document.create_element("tr")?;
        tr.set_attribute("id", format!("tr-{}", c.id).as_str())?;
        let td = document.create_element("td")?;
        td.set_text_content(Some(format!("{}", c.id).as_str()));
        tr.append_child(&td)?;

        let td = document.create_element("td")?;
        td.set_text_content(Some(c.name.as_str()));
        tr.append_child(&td)?;

        let td = document.create_element("td")?;
        td.set_text_content(Some(c.time.format("%Y-%m-%d").to_string().as_str()));
        tr.append_child(&td)?;

        let td = document.create_element("td")?;
        if let Some(desc) = c.description.clone() {
            td.set_text_content(Some(desc.as_str()));
        }
        tr.append_child(&td)?;

        let td = document.create_element("td")?;
        // 这里创建出来的是Element, 但如果要在浏览器使用 wasm 或者在wasm中使用浏览器中其他的 web 类型
        // 需要在 cargo.toml 中添加对应的类型
        // HtmlButtonElement 上才有绑定事件的功能, dyn_into 用于将 Element 转换为对应的其他 Element
        // ? dyn_into 来自于 wasm_bindgen::JsCast 命名空间, 需要手动引入
        let btn: HtmlButtonElement = document.create_element("button").unwrap().dyn_into::<HtmlButtonElement>().unwrap();
        let cid = c.id;
        let click_closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
            // 闭包, 对应的是按钮点击事件
            // 调用 window.confirm函数
            let r = confirm(format!("确认删除 ID 为 {} 的课程?", cid).as_str());
            match r {
                true => {
                    // ? spawn_local 生成一个在当前线程内运行，一定不会被偷到其它线程中运行的异步任务
                    // 内部应该是一个异步函数, 但是 delete_course 没有加上 await, 所以内部是一个 future, 是还未确定的
                    // 这里使用 spawn_local 包裹, 就会将 delete_course 这个 future 的执行放到当前线程
                    spawn_local(delete_course(1, cid));
                    alert("删除成功!");

                    // 刷新一下
                    web_sys::window().unwrap().location().reload().unwrap();
                }
                _ => {}
            }
        }) as Box<dyn Fn(_)>);
        // add_event_listener_with_callback 的第二个参数要求传递的是 function 类型的引用
        // 这里需要将闭包转换为 function 类型
        // 首先使用 Box::new 创建一个 智能指针, 在使用 Closure::wrap 一下这个智能指针, 转换为 Box<T>, T实现了 dyn Fn(_) 这个trait
        // 然后调用 as_ref().unchecked_ref(), 将闭包转换为一个 function 类型的引用
        btn.add_event_listener_with_callback("click", click_closure.as_ref().unchecked_ref()).unwrap();
        // 调用 forget 方法, 防止走出作用域被时, 回调函数被回收(这样不会回收, 但是回调函数常驻内存可能会造成内存泄漏)
        click_closure.forget();
        btn.set_attribute("class", "btn btn-danger btn-sm")?;
        btn.set_text_content(Some("Delete"));
        td.append_child(&btn)?;
        tr.append_child(&td)?;

        left_tbody.append_child(&tr)?;
    }

    Ok(())
}
