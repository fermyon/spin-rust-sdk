use spin_sdk::http::{IntoResponse, Response};
use spin_sdk::http_component;
use spin_sdk::observe;

/// A simple Spin HTTP component.
#[http_component]
async fn hello_world(_req: http::Request<()>) -> anyhow::Result<impl IntoResponse> {
    let span = observe::Span::enter("guest_span");
    std::thread::sleep(std::time::Duration::from_millis(500));
    sleep_for(5000);
    span.close();
    Ok(Response::new(200, "Hello, world!"))
}

// use spin_sdk::http::{IntoResponse, Request, Response};
// use spin_sdk::http_component;
// use spin_sdk::key_value::Store;
// use spin_sdk::observe;

// /// A simple Spin HTTP component.
// #[http_component]
// async fn hello_world(_req: http::Request<()>) -> anyhow::Result<impl IntoResponse> {
//     let span = observe::Span::enter("guest_span");
//     // span.set_attribute("foo", "bar");

//     // std::thread::sleep(std::time::Duration::from_millis(20000));

//     sleep_for(20000);

//     // do_kv_stuff()?;

//     // let resp: Response = spin_sdk::http::send(Request::get(
//     //     "https://random-data-api.fermyon.app/animals/json",
//     // ))
//     // .await?;
//     // let resp = resp
//     //     .into_builder()
//     //     .header("spin-component", "rust-outbound-http")
//     //     .build();
//     // println!("{resp:?}");

//     // do_kv_stuff()?;

//     span.close();
//     Ok(Response::new(200, "Hello, world!"))
// }

// fn do_kv_stuff() -> anyhow::Result<()> {
//     let span = observe::Span::enter("do_kv_stuff");
//     let store = Store::open_default()?;
//     store.set("hello", String::from("asdf").as_bytes())?;
//     let val = store.get("hello")?;
//     println!("val: {:?}", val);
//     span.close();
//     Ok(())
// }

fn sleep_for(x: u64) {
    let span = observe::Span::enter("sleep_for");
    std::thread::sleep(std::time::Duration::from_millis(x));
    span.close();
}
