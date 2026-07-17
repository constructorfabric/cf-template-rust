use crate::api::rest::dto::{CreateProductReq, ProductDto, UpdateProductReq};

use toolkit::api::canonical_prelude::*;
use toolkit::api::select::{apply_select, page_to_projected_json};

mod product;

// ==================== Product Handlers ====================

#[cfg(feature = "odata")]
pub(crate) use product::create_product;
#[cfg(feature = "odata")]
pub(crate) use product::delete_product;
#[cfg(feature = "odata")]
pub(crate) use product::get_product;
#[cfg(feature = "odata")]
pub(crate) use product::list_product;
#[cfg(feature = "odata")]
pub(crate) use product::update_product;
