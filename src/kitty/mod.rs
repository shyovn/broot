mod image_renderer;

pub use image_renderer::*;

use {
    once_cell::sync::Lazy,
    std::sync::Mutex,
};

static RENDERER: Lazy<Option<Mutex<KittyImageRenderer>>> = Lazy::new(|| {
    KittyImageRenderer::new().map(Mutex::new)
});

// TODO try to find another way (making app_context mut ?) to pass this
// around without the mutex gymnastic, and also to make it really lazy
// (ie only initialized when an image must be rendered)
pub fn image_renderer() -> &'static Option<Mutex<KittyImageRenderer>> {
    &*RENDERER
}

