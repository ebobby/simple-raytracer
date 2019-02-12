#[derive(Debug)]
pub struct Options {
    pub max_rays: u8,
    pub gamma: f64,
    pub diffuse: bool,
    pub specular: bool,
    pub shadows: bool,
    pub reflections: bool,
}
