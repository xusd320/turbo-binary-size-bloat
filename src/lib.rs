#![feature(arbitrary_self_types_pointers)]

use turbo_tasks::Vc;

#[turbo_tasks::value]
pub struct TurboType;

#[turbo_tasks::function]
pub fn turbo_function() -> Vc<TurboType> {
    TurboType.cell()
}

pub struct NormalType;

pub fn function() -> NormalType {
    NormalType
}
