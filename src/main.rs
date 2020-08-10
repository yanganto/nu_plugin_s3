use nu_plugin::serve_plugin;
use nu_plugin_s3::handle;

fn main() {
    serve_plugin(&mut handle::Handle::new())
}
