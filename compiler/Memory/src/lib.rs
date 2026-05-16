mod arena;
mod bumper;
mod mono;
mod redeclarable;
mod registry;

#[doc(inline)]
pub use ::bumpalo::collections::{String as ArenaString, Vec as ArenaVec};

use self::registry::Registry;
pub use self::{
  arena::{Arena, CollectIn},
  bumper::BumpAllocator,
  mono::DedicatedBumper,
  redeclarable::{IntrusiveRedeclarableLink, Redeclarable, RedeclarableIter},
};
