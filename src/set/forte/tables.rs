use std::num::NonZeroU8;
use crate::PitchClass;
use crate::set::forte::SetClass;
use crate::set::PitchClassSet;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Entry {
    pub cardinality: u8,
    pub index: NonZeroU8,
    pub z_related: Option<NonZeroU8>,
    pub prime_form: PitchClassSet,
}

const fn pcset_from_prime(prime_form: &[u8]) -> PitchClassSet {
    if prime_form.is_empty() {
        return PitchClassSet::EMPTY;
    }

    if prime_form[0] != 0 {
        panic!("Prime form must start with 0");
    }

    let mut pcset = PitchClassSet::EMPTY;
    pcset = pcset.with_set(PitchClass::C);

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
        index: NonZeroU8::new(index).expect("index can't be zero"),
        z_related: None,
        prime_form: pcset_from_prime(prime_form),
    }
}

const fn entry_z(cardinality: u8, index: u8, z_index: u8, prime_form: &[u8]) -> Entry {
    Entry {
        cardinality,
        index: NonZeroU8::new(index).expect("index can't be zero"),
        z_related: Some(NonZeroU8::new(z_index).expect("z_index must be non-zero")),
        prime_form: pcset_from_prime(prime_form),
    }
}

// TODO(const): ideally this becomes a 'binary_search_by' call, but it's not a big deal at all
pub const fn lookup(cardinality: u8, index: u8) -> Entry {
    assert!(
        cardinality <= 12 && 0 < index && index <= SetClass::max_index(cardinality).expect("valid cardinality"),
        "lookup should be with valid cardinality and index",
    );

    let mut i = OFFSET[cardinality as usize];
    let next_first = OFFSET[cardinality as usize + 1];

    while i < next_first {
        let entry = TABLE[i];

        debug_assert!(
            entry.cardinality == cardinality,
            "should be indexing in the right place",
        );

        if entry.index.get() == index {
            return entry;
        }

        i += 1;
    }

    panic!("unreachable!: lookup should've found entry");
}

const TABLE: [Entry; 224] = [
    // cardinality 0
    entry(0, 1, &[]),
    // cardinality 1
    entry(1, 1, &[0]),
    // cardinality 2
    entry(2, 1, &[0, 1]),
    entry(2, 2, &[0, 2]),
    entry(2, 3, &[0, 3]),
    entry(2, 4, &[0, 4]),
    entry(2, 5, &[0, 5]),
    entry(2, 6, &[0, 6]),
    // cardinality 3
    entry(3, 1, &[0, 1, 2]),
    entry(3, 2, &[0, 1, 3]),
    entry(3, 3, &[0, 1, 4]),
    entry(3, 4, &[0, 1, 5]),
    entry(3, 5, &[0, 1, 6]),
    entry(3, 6, &[0, 2, 4]),
    entry(3, 7, &[0, 2, 5]),
    entry(3, 8, &[0, 2, 6]),
    entry(3, 9, &[0, 2, 7]),
    entry(3, 10, &[0, 3, 6]),
    entry(3, 11, &[0, 3, 7]),
    entry(3, 12, &[0, 4, 8]),
    // cardinality 4
    entry(4, 1, &[0, 1, 2, 3]),
    entry(4, 2, &[0, 1, 2, 4]),
    entry(4, 3, &[0, 1, 3, 4]),
    entry(4, 4, &[0, 1, 2, 5]),
    entry(4, 5, &[0, 1, 2, 6]),
    entry(4, 6, &[0, 1, 2, 7]),
    entry(4, 7, &[0, 1, 4, 5]),
    entry(4, 8, &[0, 1, 5, 6]),
    entry(4, 9, &[0, 1, 6, 7]),
    entry(4, 10, &[0, 2, 3, 5]),
    entry(4, 11, &[0, 1, 3, 5]),
    entry(4, 12, &[0, 2, 3, 6]),
    entry(4, 13, &[0, 1, 3, 6]),
    entry(4, 14, &[0, 2, 3, 7]),
    entry_z(4, 15, 29, &[0, 1, 4, 6]),
    entry(4, 16, &[0, 1, 5, 7]),
    entry(4, 17, &[0, 3, 4, 7]),
    entry(4, 18, &[0, 1, 4, 7]),
    entry(4, 19, &[0, 1, 4, 8]),
    entry(4, 20, &[0, 1, 5, 8]),
    entry(4, 21, &[0, 2, 4, 6]),
    entry(4, 22, &[0, 2, 4, 7]),
    entry(4, 23, &[0, 2, 5, 7]),
    entry(4, 24, &[0, 2, 4, 8]),
    entry(4, 25, &[0, 2, 6, 8]),
    entry(4, 26, &[0, 3, 5, 8]),
    entry(4, 27, &[0, 2, 5, 8]),
    entry(4, 28, &[0, 3, 6, 9]),
    entry_z(4, 29, 15, &[0, 1, 3, 7]),
    // cardinality 5
    entry(5, 1, &[0, 1, 2, 3, 4]),
    entry(5, 2, &[0, 1, 2, 3, 5]),
    entry(5, 3, &[0, 1, 2, 4, 5]),
    entry(5, 4, &[0, 1, 2, 3, 6]),
    entry(5, 5, &[0, 1, 2, 3, 7]),
    entry(5, 6, &[0, 1, 2, 5, 6]),
    entry(5, 7, &[0, 1, 2, 6, 7]),
    entry(5, 8, &[0, 2, 3, 4, 6]),
    entry(5, 9, &[0, 1, 2, 4, 6]),
    entry(5, 10, &[0, 1, 3, 4, 6]),
    entry(5, 11, &[0, 2, 3, 4, 7]),
    entry_z(5, 12, 36, &[0, 1, 3, 5, 6]),
    entry(5, 13, &[0, 1, 2, 4, 8]),
    entry(5, 14, &[0, 1, 2, 5, 7]),
    entry(5, 15, &[0, 1, 2, 6, 8]),
    entry(5, 16, &[0, 1, 3, 4, 7]),
    entry_z(5, 17, 37, &[0, 1, 3, 4, 8]),
    entry_z(5, 18, 38, &[0, 1, 4, 5, 7]),
    entry(5, 19, &[0, 1, 3, 6, 7]),
    entry(5, 20, &[0, 1, 3, 7, 8]),
    entry(5, 21, &[0, 1, 4, 5, 8]),
    entry(5, 22, &[0, 1, 4, 7, 8]),
    entry(5, 23, &[0, 2, 3, 5, 7]),
    entry(5, 24, &[0, 1, 3, 5, 7]),
    entry(5, 25, &[0, 2, 3, 5, 8]),
    entry(5, 26, &[0, 2, 4, 5, 8]),
    entry(5, 27, &[0, 1, 3, 5, 8]),
    entry(5, 28, &[0, 2, 3, 6, 8]),
    entry(5, 29, &[0, 1, 3, 6, 8]),
    entry(5, 30, &[0, 1, 4, 6, 8]),
    entry(5, 31, &[0, 1, 3, 6, 9]),
    entry(5, 32, &[0, 1, 4, 6, 9]),
    entry(5, 33, &[0, 2, 4, 6, 8]),
    entry(5, 34, &[0, 2, 4, 6, 9]),
    entry(5, 35, &[0, 2, 4, 7, 9]),
    entry_z(5, 36, 12, &[0, 1, 2, 4, 7]),
    entry_z(5, 37, 17, &[0, 3, 4, 5, 8]),
    entry_z(5, 38, 18, &[0, 1, 2, 5, 8]),
    // cardinality 6
    entry(6, 1, &[0, 1, 2, 3, 4, 5]),
    entry(6, 2, &[0, 1, 2, 3, 4, 6]),
    entry_z(6, 3, 36, &[0, 1, 2, 3, 5, 6]),
    entry_z(6, 4, 37, &[0, 1, 2, 4, 5, 6]),
    entry(6, 5, &[0, 1, 2, 3, 6, 7]),
    entry_z(6, 6, 38, &[0, 1, 2, 5, 6, 7]),
    entry(6, 7, &[0, 1, 2, 6, 7, 8]),
    entry(6, 8, &[0, 2, 3, 4, 5, 7]),
    entry(6, 9, &[0, 1, 2, 3, 5, 7]),
    entry_z(6, 10, 39, &[0, 1, 3, 4, 5, 7]),
    entry_z(6, 11, 40, &[0, 1, 2, 4, 5, 7]),
    entry_z(6, 12, 41, &[0, 1, 2, 4, 6, 7]),
    entry_z(6, 13, 42, &[0, 1, 3, 4, 6, 7]),
    entry(6, 14, &[0, 1, 3, 4, 5, 8]),
    entry(6, 15, &[0, 1, 2, 4, 5, 8]),
    entry(6, 16, &[0, 1, 4, 5, 6, 8]),
    entry_z(6, 17, 43, &[0, 1, 2, 4, 7, 8]),
    entry(6, 18, &[0, 1, 2, 5, 7, 8]),
    entry_z(6, 19, 44, &[0, 1, 3, 4, 7, 8]),
    entry(6, 20, &[0, 1, 4, 5, 8, 9]),
    entry(6, 21, &[0, 2, 3, 4, 6, 8]),
    entry(6, 22, &[0, 1, 2, 4, 6, 8]),
    entry_z(6, 23, 45, &[0, 2, 3, 5, 6, 8]),
    entry_z(6, 24, 46, &[0, 1, 3, 4, 6, 8]),
    entry_z(6, 25, 47, &[0, 1, 3, 5, 6, 8]),
    entry_z(6, 26, 48, &[0, 1, 3, 5, 7, 8]),
    entry(6, 27, &[0, 1, 3, 4, 6, 9]),
    entry_z(6, 28, 49, &[0, 1, 3, 5, 6, 9]),
    entry_z(6, 29, 50, &[0, 1, 3, 6, 8, 9]),
    entry(6, 30, &[0, 1, 3, 6, 7, 9]),
    entry(6, 31, &[0, 1, 3, 5, 8, 9]),
    entry(6, 32, &[0, 2, 4, 5, 7, 9]),
    entry(6, 33, &[0, 2, 3, 5, 7, 9]),
    entry(6, 34, &[0, 1, 3, 5, 7, 9]),
    entry(6, 35, &[0, 2, 4, 6, 8, 10]),
    entry_z(6, 36, 3, &[0, 1, 2, 3, 4, 7]),
    entry_z(6, 37, 4, &[0, 1, 2, 3, 4, 8]),
    entry_z(6, 38, 6, &[0, 1, 2, 3, 7, 8]),
    entry_z(6, 39, 10, &[0, 2, 3, 4, 5, 8]),
    entry_z(6, 40, 11, &[0, 1, 2, 3, 5, 8]),
    entry_z(6, 41, 12, &[0, 1, 2, 3, 6, 8]),
    entry_z(6, 42, 13, &[0, 1, 2, 3, 6, 9]),
    entry_z(6, 43, 17, &[0, 1, 2, 5, 6, 8]),
    entry_z(6, 44, 19, &[0, 1, 2, 5, 6, 9]),
    entry_z(6, 45, 23, &[0, 2, 3, 4, 6, 9]),
    entry_z(6, 46, 24, &[0, 1, 2, 4, 6, 9]),
    entry_z(6, 47, 25, &[0, 1, 2, 4, 7, 9]),
    entry_z(6, 48, 26, &[0, 1, 2, 5, 7, 9]),
    entry_z(6, 49, 28, &[0, 1, 3, 4, 7, 9]),
    entry_z(6, 50, 29, &[0, 1, 4, 6, 7, 9]),
    // cardinality 7
    entry(7, 1, &[0, 1, 2, 3, 4, 5, 6]),
    entry(7, 2, &[0, 1, 2, 3, 4, 5, 7]),
    entry(7, 3, &[0, 1, 2, 3, 4, 5, 8]),
    entry(7, 4, &[0, 1, 2, 3, 4, 6, 7]),
    entry(7, 5, &[0, 1, 2, 3, 5, 6, 7]),
    entry(7, 6, &[0, 1, 2, 3, 4, 7, 8]),
    entry(7, 7, &[0, 1, 2, 3, 6, 7, 8]),
    entry(7, 8, &[0, 2, 3, 4, 5, 6, 8]),
    entry(7, 9, &[0, 1, 2, 3, 4, 6, 8]),
    entry(7, 10, &[0, 1, 2, 3, 4, 6, 9]),
    entry(7, 11, &[0, 1, 3, 4, 5, 6, 8]),
    entry_z(7, 12, 36, &[0, 1, 2, 3, 4, 7, 9]),
    entry(7, 13, &[0, 1, 2, 4, 5, 6, 8]),
    entry(7, 14, &[0, 1, 2, 3, 5, 7, 8]),
    entry(7, 15, &[0, 1, 2, 4, 6, 7, 8]),
    entry(7, 16, &[0, 1, 2, 3, 5, 6, 9]),
    entry_z(7, 17, 37, &[0, 1, 2, 4, 5, 6, 9]),
    entry_z(7, 18, 38, &[0, 1, 2, 3, 5, 8, 9]),
    entry(7, 19, &[0, 1, 2, 3, 6, 7, 9]),
    entry(7, 20, &[0, 1, 2, 4, 7, 8, 9]),
    entry(7, 21, &[0, 1, 2, 4, 5, 8, 9]),
    entry(7, 22, &[0, 1, 2, 5, 6, 8, 9]),
    entry(7, 23, &[0, 2, 3, 4, 5, 7, 9]),
    entry(7, 24, &[0, 1, 2, 3, 5, 7, 9]),
    entry(7, 25, &[0, 2, 3, 4, 6, 7, 9]),
    entry(7, 26, &[0, 1, 3, 4, 5, 7, 9]),
    entry(7, 27, &[0, 1, 2, 4, 5, 7, 9]),
    entry(7, 28, &[0, 1, 3, 5, 6, 7, 9]),
    entry(7, 29, &[0, 1, 2, 4, 6, 7, 9]),
    entry(7, 30, &[0, 1, 2, 4, 6, 8, 9]),
    entry(7, 31, &[0, 1, 3, 4, 6, 7, 9]),
    entry(7, 32, &[0, 1, 3, 4, 6, 8, 9]),
    entry(7, 33, &[0, 1, 2, 4, 6, 8, 10]),
    entry(7, 34, &[0, 1, 3, 4, 6, 8, 10]),
    entry(7, 35, &[0, 1, 3, 5, 6, 8, 10]),
    entry_z(7, 36, 12, &[0, 1, 2, 3, 5, 6, 8]),
    entry_z(7, 37, 17, &[0, 1, 3, 4, 5, 7, 8]),
    entry_z(7, 38, 18, &[0, 1, 2, 4, 5, 7, 8]),
    // cardinality 8
    entry(8, 1, &[0, 1, 2, 3, 4, 5, 6, 7]),
    entry(8, 2, &[0, 1, 2, 3, 4, 5, 6, 8]),
    entry(8, 3, &[0, 1, 2, 3, 4, 5, 6, 9]),
    entry(8, 4, &[0, 1, 2, 3, 4, 5, 7, 8]),
    entry(8, 5, &[0, 1, 2, 3, 4, 6, 7, 8]),
    entry(8, 6, &[0, 1, 2, 3, 5, 6, 7, 8]),
    entry(8, 7, &[0, 1, 2, 3, 4, 5, 8, 9]),
    entry(8, 8, &[0, 1, 2, 3, 4, 7, 8, 9]),
    entry(8, 9, &[0, 1, 2, 3, 6, 7, 8, 9]),
    entry(8, 10, &[0, 2, 3, 4, 5, 6, 7, 9]),
    entry(8, 11, &[0, 1, 2, 3, 4, 5, 7, 9]),
    entry(8, 12, &[0, 1, 3, 4, 5, 6, 7, 9]),
    entry(8, 13, &[0, 1, 2, 3, 4, 6, 7, 9]),
    entry(8, 14, &[0, 1, 2, 4, 5, 6, 7, 9]),
    entry_z(8, 15, 29, &[0, 1, 2, 3, 4, 6, 8, 9]),
    entry(8, 16, &[0, 1, 2, 3, 5, 7, 8, 9]),
    entry(8, 17, &[0, 1, 3, 4, 5, 6, 8, 9]),
    entry(8, 18, &[0, 1, 2, 3, 5, 6, 8, 9]),
    entry(8, 19, &[0, 1, 2, 4, 5, 6, 8, 9]),
    entry(8, 20, &[0, 1, 2, 4, 5, 7, 8, 9]),
    entry(8, 21, &[0, 1, 2, 3, 4, 6, 8, 10]),
    entry(8, 22, &[0, 1, 2, 3, 5, 6, 8, 10]),
    entry(8, 23, &[0, 1, 2, 3, 5, 7, 8, 10]),
    entry(8, 24, &[0, 1, 2, 4, 5, 6, 8, 10]),
    entry(8, 25, &[0, 1, 2, 4, 6, 7, 8, 10]),
    entry(8, 26, &[0, 1, 2, 4, 5, 7, 9, 10]),
    entry(8, 27, &[0, 1, 2, 4, 5, 7, 8, 10]),
    entry(8, 28, &[0, 1, 3, 4, 6, 7, 9, 10]),
    entry_z(8, 29, 15, &[0, 1, 2, 3, 5, 6, 7, 9]),
    // cardinality 9
    entry(9, 1, &[0, 1, 2, 3, 4, 5, 6, 7, 8]),
    entry(9, 2, &[0, 1, 2, 3, 4, 5, 6, 7, 9]),
    entry(9, 3, &[0, 1, 2, 3, 4, 5, 6, 8, 9]),
    entry(9, 4, &[0, 1, 2, 3, 4, 5, 7, 8, 9]),
    entry(9, 5, &[0, 1, 2, 3, 4, 6, 7, 8, 9]),
    entry(9, 6, &[0, 1, 2, 3, 4, 5, 6, 8, 10]),
    entry(9, 7, &[0, 1, 2, 3, 4, 5, 7, 8, 10]),
    entry(9, 8, &[0, 1, 2, 3, 4, 6, 7, 8, 10]),
    entry(9, 9, &[0, 1, 2, 3, 5, 6, 7, 8, 10]),
    entry(9, 10, &[0, 1, 2, 3, 4, 6, 7, 9, 10]),
    entry(9, 11, &[0, 1, 2, 3, 5, 6, 7, 9, 10]),
    entry(9, 12, &[0, 1, 2, 4, 5, 6, 8, 9, 10]),
    // cardinality 10
    entry(10, 1, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]),
    entry(10, 2, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 10]),
    entry(10, 3, &[0, 1, 2, 3, 4, 5, 6, 7, 9, 10]),
    entry(10, 4, &[0, 1, 2, 3, 4, 5, 6, 8, 9, 10]),
    entry(10, 5, &[0, 1, 2, 3, 4, 5, 7, 8, 9, 10]),
    entry(10, 6, &[0, 1, 2, 3, 4, 6, 7, 8, 9, 10]),
    // cardinality 11
    entry(11, 1, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
    // cardinality 12
    entry(12, 1, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]),
];

const OFFSET: [usize; 14] = const {
    let mut offsets = [0; 14];

    let mut next_cardinality = 0;
    let mut i = 0;

    while next_cardinality <= 12 {
        if TABLE[i].cardinality == next_cardinality {
            offsets[next_cardinality as usize] = i;
            next_cardinality += 1;
        }

        i += 1;
    }

    offsets[13] = TABLE.len();

    offsets
};

pub fn lookup_prime_form(prime_form: PitchClassSet) -> Option<Entry> {
    let cardinality = prime_form.len();
    let start = OFFSET[cardinality as usize];
    let end = OFFSET[cardinality as usize + 1];

    TABLE[start..end].iter()
        .find(|ent| ent.prime_form == prime_form)
        .copied()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use super::*;

    fn all_pcsets() -> impl Iterator<Item=PitchClassSet> + Clone {
        (0..=0xfff).map(PitchClassSet::from_bits_masked)
    }

    #[test]
    fn offsets_valid() {
        for cardinality in 0..=12 {
            let start = OFFSET[cardinality];
            let end = OFFSET[cardinality + 1];

            assert!(
                TABLE[start..end].iter().all(|ent| ent.cardinality == cardinality as u8),
                "offsets misaligned"
            );
        }
    }

    #[test]
    fn prime_forms() {
        let mut prime_forms = HashSet::with_capacity(TABLE.len());

        for pcset in all_pcsets() {
            prime_forms.insert(pcset.prime_form());
        }

        assert_eq!(
            prime_forms.len(), TABLE.len(),
            "table should cover exactly all prime forms",
        );

        for prime_form in prime_forms {
            let lookup = lookup_prime_form(prime_form);

            assert_eq!(
                lookup,
                TABLE.iter().find(|ent| ent.prime_form == prime_form).copied(),
                "lookup prime form optimization should find all values"
            );

            assert!(
                lookup.is_some(),
                "lookup failed for {}", prime_form.display_chromas()
            );
        }
    }
}