use std::num::NonZeroU8;
use crate::PitchClass;
use crate::set::PitchClassSet;

struct Entry {
    cardinality: u8,
    index: u8,
    z_related: Option<(NonZeroU8)>,
    prime_form: PitchClassSet,
}

const fn pcset_from_prime(prime_form: &[u8]) -> PitchClassSet {
    let mut pcset = PitchClassSet::EMPTY;

    if prime_form.is_empty() || prime_form[0] != 0 {
        panic!("Prime form must start with 0")
    }

    let mut last = 0;

    let mut i = 1;
    while i < prime_form.len() {
        let chroma = prime_form[i];

        let Some(pc) = PitchClass::from_chroma(chroma) else {
            panic!("invalid pc chroma");
        };

        if i != 0 && last >= chroma {
            panic!("prime form not strictly ascending");
        }

        last = chroma;
        pcset = pcset.with_set(pc);

        i += 1;
    }

    pcset
}

const fn entry(cardinality: u8, index: u8, prime_form: &[u8]) -> Entry {
    Entry {
        cardinality,
        index,
        z_related: None,
        prime_form: pcset_from_prime(prime_form),
    }
}

const fn entry_z(cardinality: u8, index: u8, z_index: u8, prime_form: &[u8]) -> Entry {
    Entry {
        cardinality,
        index,
        z_related: Some(NonZeroU8::new(z_index).expect("z_index must be non-zero")),
        prime_form: pcset_from_prime(prime_form),
    }
}