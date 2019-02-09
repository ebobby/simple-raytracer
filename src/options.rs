#[derive(Debug)]
pub struct Options {
    pub bias: f64,
    pub max_rays: u8,
    pub diffuse: bool,
    pub specular: bool,
    pub shadows: bool,
    pub reflections: bool,
}
