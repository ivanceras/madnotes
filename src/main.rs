//#![deny(warnings)]
use futures::stream::StreamExt;
use futures::TryStreamExt;
use hyper::Body;
use native_dialog::FileDialog;
use route::Route;
use sauron::Render;
use std::sync::Mutex;
use tokio::sync::oneshot;
use web_view::*;

mod route;
mod serve_files;
mod server;

//TODO: use the license verifier here
// The file is stored in ~/.config/
fn check_and_verify_license() -> bool {
    false
}

#[tokio::main]
async fn main() {
    let settings = client::Settings::default();
    let app_title = if check_and_verify_license() {
        settings.app_title.clone()
    } else {
        format!("{} (UNREGISTERED)", settings.app_title.clone())
    };

    #[cfg(feature = "open-ports")]
    let (socket_tx, socket_rx) = oneshot::channel();
    #[cfg(feature = "open-ports")]
    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

    #[cfg(feature = "open-ports")]
    tokio::spawn(server::serve_ephemeral(settings, socket_tx, shutdown_rx));

    #[cfg(feature = "open-ports")]
    let socket = socket_rx.await.expect("must get the socket address");
    #[cfg(feature = "open-ports")]
    let content = {
        let url = format!("http://{}", socket);
        println!("{}", url);
        Content::Url(url)
    };

    #[cfg(not(feature = "open-ports"))]
    let content = {
        let html = server::page::index(&settings).render_to_string();
        println!("html: {}", html);
        Content::Html(html)
    };

    web_view::builder()
        .title(&app_title)
        .content(content)
        .size(1200, 600)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(move |webview, arg| invoke_handler(webview, arg))
        .run()
        .expect("must launch the web view");

    println!("shuting it down..");

    #[cfg(feature = "open-ports")]
    shutdown_tx.send(()).expect("must send the shutdown signal");
}

/// The this handles the call from javascript, which contains the serialized object Route based on
/// the route methods we then map it to their corresponding functions that needs to be done.
///
/// Along with the route is a callback_id which is essentially a pointer to a stack in the
/// javascript side, where an array of callback is maintained. This callback_id corresponds to the
/// index in the array of the callbacks which we then call along with the argument payload.
///
fn invoke_handler(webview: &mut WebView<'_, ()>, arg: &str) -> Result<(), Error> {
    match arg {
        "open" => {
            let path = FileDialog::new()
                .set_location("~/Desktop")
                .add_filter("PNG Image", &["png"])
                .add_filter("JPEG Image", &["jpg", "jpeg"])
                .show_open_single_file()
                .unwrap();
            println!("path: {:?}", path);
        }
        _ => {
            let route: Route = serde_json::from_str(arg).expect("Error decoding json");
            println!("url: {}", route.url);
            let raw_response =
                serve_files::raw_serve(&client::Settings::default(), &*route.path_and_query())
                    .expect("must not fail request");

            response_callback(
                webview,
                route.callback_id,
                base64::encode(&raw_response.content),
            );
        }
    }
    Ok(())
}

fn response_callback<'a, T>(
    webview: &mut WebView<'a, T>,
    callback_id: usize,
    response_payload: String,
) {
    webview
        .eval(&format!(
            "responseCallback({},\"{}\")",
            callback_id, response_payload
        ))
        .expect("must eval");
}
