use std::cmp::Ordering;

use crate::fst_traits::{ExpandedFst, MutableFst};
use crate::semirings::Semiring;
use crate::Tr;

/// Compare only input labels.
pub fn ilabel_compare<W: Semiring>(a: &Tr<W>, b: &Tr<W>) -> Ordering {
    a.ilabel.cmp(&b.ilabel)
}

/// Compare only output labels.
pub fn olabel_compare<W: Semiring>(a: &Tr<W>, b: &Tr<W>) -> Ordering {
    a.olabel.cmp(&b.olabel)
}

/// Sorts arcs leaving each state of the FST using a compare function
pub fn tr_sort<F>(fst: &mut F, comp: impl Fn(&Tr<F::W>, &Tr<F::W>) -> Ordering)
where
    F: MutableFst + ExpandedFst,
{
    for state in 0..fst.num_states() {
        fst.sort_trs_unchecked(state, &comp);
    }
}