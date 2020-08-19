use crate::astconv::AstConv;

use crate::middle::resolve_lifetime as rl;
use rustc_hir as hir;
use rustc_middle::ty;

pub trait RegionToRegion<'tcx>: AstConv<'tcx> {}

impl<'o, 'tcx> dyn RegionToRegion<'tcx> + 'o {}
