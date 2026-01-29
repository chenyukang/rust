//@ build-fail
//@ revisions: legacy v0
//@ compile-flags: --crate-name=c
//@[legacy]compile-flags: -C symbol-mangling-version=legacy -Z unstable-options
//@    [v0]compile-flags: -C symbol-mangling-version=v0
//@[legacy]normalize-stderr: "h[[:xdigit:]]{16}" -> "h[HASH]"
//@    [v0]normalize-stderr: "c\[.*?\]" -> "c[HASH]"
#![feature(rustc_attrs)]

pub struct Unsigned<const F: u8>;

impl Unsigned<11> {
    #[rustc_symbol_name]
    //[v0]~^ ERROR symbol-name(_RNvMCs
    //[v0]~| ERROR demangling(<c[
    //[v0]~| ERROR demangling-alt(<c::Unsigned<11>>::f)
    //[legacy]~^^^^ ERROR symbol-name(_ZN1c48Unsigned$LT$const$u20$$u7b$$u20$11$u20$$u7d$$GT$
    //[legacy]~|    ERROR demangling(c::Unsigned<const { 11 }>::f::
    //[legacy]~|    ERROR demangling-alt(c::Unsigned<const { 11 }>::f)
    fn f() {}
}

pub struct Signed<const F: i16>;

impl Signed<-152> {
    #[rustc_symbol_name]
    //[v0]~^ ERROR symbol-name(_RNvMs_Cs
    //[v0]~| ERROR demangling(<c[
    //[v0]~| ERROR demangling-alt(<c::Signed<-152>>::f)
    //[legacy]~^^^^ ERROR symbol-name(_ZN1c48Signed$LT$const$u20$$u7b$$u20$.152$u20$$u7d$$GT$
    //[legacy]~|    ERROR demangling(c::Signed<const { .152 }>::f::
    //[legacy]~|    ERROR demangling-alt(c::Signed<const { .152 }>::f)
    fn f() {}
}

pub struct Bool<const F: bool>;

impl Bool<true> {
    #[rustc_symbol_name]
    //[v0]~^ ERROR symbol-name(_RNvMs0_Cs
    //[v0]~| ERROR demangling(<c[
    //[v0]~| ERROR demangling-alt(<c::Bool<true>>::f)
    //[legacy]~^^^^ ERROR symbol-name(_ZN1c46Bool$LT$const$u20$$u7b$$u20$true$u20$$u7d$$GT$
    //[legacy]~|    ERROR demangling(c::Bool<const { true }>::f::
    //[legacy]~|    ERROR demangling-alt(c::Bool<const { true }>::f)
    fn f() {}
}

pub struct Char<const F: char>;

impl Char<'∂'> {
    #[rustc_symbol_name]
    //[v0]~^ ERROR symbol-name(_RNvMs1_Cs
    //[v0]~| ERROR demangling(<c[
    //[v0]~| ERROR demangling-alt(<c::Char<'∂'>>::f)
    //[legacy]~^^^^ ERROR symbol-name(_ZN1c59Char$LT$const$u20$$u7b$$u20$$u27$$u2202$$u27$$u20$$u7d$$GT$
    //[legacy]~|    ERROR demangling(c::Char<const { '∂' }>::f::
    //[legacy]~|    ERROR demangling-alt(c::Char<const { '∂' }>::f)
    fn f() {}
}

fn main() {}
