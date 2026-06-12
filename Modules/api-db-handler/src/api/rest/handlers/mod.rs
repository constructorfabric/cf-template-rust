use crate::api::rest::dto::PokemonDto;

use toolkit::api::canonical_prelude::*;
use toolkit::api::select::{apply_select, page_to_projected_json};

mod pokemon;

// ==================== Pokemon Handlers ====================

#[cfg(feature = "odata")]
pub(crate) use pokemon::get_pokemon;
#[cfg(feature = "odata")]
pub(crate) use pokemon::list_pokemon;
