pub use self::interleaved::DrawFlat;
pub use self::separate::DrawFlatSeparate;

mod interleaved;
mod separate;

use pass::util::TextureType;

static VERT_SRC: &[u8] = include_bytes!("../shaders/vertex/basic.glsl");
static FRAG_SRC: &[u8] = include_bytes!("../shaders/fragment/flat.glsl");

static TEXTURES: [TextureType; 1] = [TextureType::Albedo];
