mod console;
mod doryen;

use doryen_rs::DoryenApi;

pub fn render_console() -> console::StdoutRender {
    console::StdoutRender
}

pub fn render_doryen(doryen_api: &mut DoryenApi) -> doryen::DoryenRenderer {
    doryen::DoryenRenderer { doryen_api }
}
