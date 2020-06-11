
use web_view::*;

#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    println!("Using input file: {}", matches.value_of("steam_id").unwrap());

    web_view::builder()
        .title("")
        .content(Content::Html(HTML))
        .size(150, 150)
        .frameless(true)
        .debug(true)
        .user_data("")
        .invoke_handler(|webview, arg| {
            match arg {
                "exit" => webview.exit(),
                _ => (),
            }
            Ok(())
        })
        .run()
        .unwrap();
}

const HTML: &str = r#"
<!doctype html>
<html>
  <body>
        <button onclick="external.invoke('exit')" style="display:block;width:100px;height:100px;font-size:24pt;margin:25px auto;">exit</button>
  </body>
</html>
"#;