#![allow(dead_code)]
use std::ptr::PtrExt;

pub enum RefMatch {
    Match(&'static [u8]),
    Partial(&'static Trie),
    Mismatch,
}

pub struct Trie(&'static [TP], u16, u8);

type P = *const Trie;
pub struct TP(*const Trie);
unsafe impl Sync for TP {}
unsafe impl Send for TP {}
const N: TP = TP(0 as *const _);


pub fn get_named_ref(name: &[u8]) -> Option<&'static [u8]> {
    if name.len() == 0 {
        return None
    }
    match match_ref(name) {
        RefMatch::Match(r) => Some(r),
        _                  => None
    }
}

pub fn match_ref(name: &[u8]) -> RefMatch {
    match_ref_continue(name, &TRIE)
}

pub fn match_ref_continue(name: &[u8], begin: &'static Trie) -> RefMatch {
    let mut begin = begin;
    for i in 0..name.len() {
        let letter = unsafe { *name.get_unchecked(i) };
        if begin.2 <= letter {
            let idx = (letter - begin.2) as usize;
            let tp = if begin.0.len() > idx {
                unsafe { begin.0.get_unchecked(idx).0.as_ref() }
            } else {
                return RefMatch::Mismatch;
            };
            begin = if let Some(t) = tp {
                t
            } else {
                return RefMatch::Mismatch;
            };
        } else {
            return RefMatch::Mismatch;
        }
    }
    if begin.1 == 0xffff {
        RefMatch::Partial(begin)
    } else {
        return RefMatch::Match(unsafe { DECODED.get_unchecked(begin.1 as usize) });
    }
}

include!("unescape_named_gen.rs");
