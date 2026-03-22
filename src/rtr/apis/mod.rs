use crate::rtr::runtime::memory::Memory;
use crate::rtr::runtime::scope::Scope;
use crate::rtr::apis::rwl::inject as rwl_inject;

pub mod rwl;

pub fn inject(memory: &mut Memory, scope: &mut Scope) {
    rwl_inject(memory, scope);
}
