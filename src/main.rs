use hatter::Env as Hatter;
use vial::prelude::*;

routes! {
    GET "/:path" => |req| -> vial::Result<Response> {
        let name = req.path().trim_start_matches('/').replace("..", ".");
        let file = format!("{}.hat", name);
        if asset::exists(&file) {
            let mut env = Hatter::new();
            Ok(env.render(&asset::to_string(&file)?).to_response())
        } else {
            Ok(Response::from(404))
        }
    };

    GET "/" => |_| "index";
}

fn main() {
    let mut args = std::env::args().skip(1);
    let mut path = ".".to_owned();
    if let Some(dir) = args.next() {
        path = dir;
    }
    let addr = "0.0.0.0:8185";
    let banner = format!("\x1b[1m🎩 Serving {} at http://{} \x1b[0m", path, addr);
    vial::asset_dir!(path);
    vial::run_with_banner!(&banner, addr).unwrap();
}
