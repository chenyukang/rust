use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::BuildHasherDefault;
use std::marker::PhantomData;

struct Map<K, S = Vec<K>>(S, PhantomData<K>);

fn make_map<K, S>(hasher: S) -> Map<K, S> {
    Map(hasher, PhantomData)
}

struct Builder<K>(PhantomData<K>);

impl<K> Builder<K> {
    fn with_hasher<S>(self, hasher: S) -> Map<K, S> {
        Map(hasher, PhantomData)
    }
}

fn main() {
    let _: HashMap<(), ()> =
        HashMap::with_hasher(BuildHasherDefault::<DefaultHasher>::new());
    //~^ ERROR mismatched types

    let _: Map<()> = make_map(String::new());
    //~^ ERROR mismatched types

    let _: Map<()> = Builder::<()>(PhantomData).with_hasher(String::new());
    //~^ ERROR mismatched types
}
