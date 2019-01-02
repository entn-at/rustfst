use std::collections::VecDeque;

use crate::fst_traits::{CoreFst, ExpandedFst};
use crate::semirings::Semiring;
use crate::{Result, StateId};

/// This operation computes the shortest distance from the state `state_id` to every state.
/// The shortest distance from `p` to `q` is the ⊕-sum of the weights
/// of all the paths between `p` and `q`.
///
/// # Example
/// ```
/// # use rustfst::semirings::{Semiring, IntegerWeight};
/// # use rustfst::fst_impls::VectorFst;
/// # use rustfst::fst_traits::MutableFst;
/// # use rustfst::single_source_shortest_distance;
/// # use rustfst::Arc;
/// let mut fst = VectorFst::new();
/// let s0 = fst.add_state();
/// let s1 = fst.add_state();
/// let s2 = fst.add_state();
///
/// fst.set_start(s0).unwrap();
/// fst.add_arc(s0, Arc::new(32, 23, IntegerWeight::new(18), s1));
/// fst.add_arc(s0, Arc::new(32, 23, IntegerWeight::new(21), s2));
/// fst.add_arc(s1, Arc::new(32, 23, IntegerWeight::new(55), s2));
///
/// let dists = single_source_shortest_distance(&fst, s1).unwrap();
///
/// assert_eq!(dists, vec![
///     IntegerWeight::ZERO,
///     IntegerWeight::ONE,
///     IntegerWeight::new(55),
/// ]);
///
/// ```
pub fn single_source_shortest_distance<F: ExpandedFst>(
    fst: &F,
    state_id: StateId,
) -> Result<Vec<<F as CoreFst>::W>> {
    let num_states = fst.num_states();

    let mut d = vec![];
    let mut r = vec![];

    d.resize(num_states, <F as CoreFst>::W::ZERO);
    r.resize(num_states, <F as CoreFst>::W::ZERO);

    // Check whether the wFST contains the state
    if state_id < fst.num_states() {
        d[state_id] = <F as CoreFst>::W::ONE;
        r[state_id] = <F as CoreFst>::W::ONE;

        let mut queue = VecDeque::new();
        queue.push_back(state_id);

        while !queue.is_empty() {
            let state_cour = queue.pop_front().unwrap();
            let r2 = r[state_cour];
            r[state_cour] = <F as CoreFst>::W::ZERO;

            for arc in fst.arcs_iter(state_cour)? {
                let nextstate = arc.nextstate;
                if d[nextstate] != d[nextstate].plus(&r2.times(&arc.weight)) {
                    d[nextstate] = d[nextstate].plus(&r2.times(&arc.weight));
                    r[nextstate] = r[nextstate].plus(&r2.times(&arc.weight));
                    if !queue.contains(&nextstate) {
                        queue.push_back(nextstate);
                    }
                }
            }
        }
    }

    Ok(d)
}

/// This operation computes the shortest distance from the initial state to every state.
/// The shortest distance from `p` to `q` is the ⊕-sum of the weights
/// of all the paths between `p` and `q`.
///
/// # Example
/// ```
/// # use rustfst::semirings::{Semiring, IntegerWeight};
/// # use rustfst::fst_impls::VectorFst;
/// # use rustfst::fst_traits::MutableFst;
/// # use rustfst::shortest_distance;
/// # use rustfst::Arc;
/// let mut fst = VectorFst::new();
/// let s0 = fst.add_state();
/// let s1 = fst.add_state();
/// let s2 = fst.add_state();
///
/// fst.set_start(s0).unwrap();
/// fst.add_arc(s0, Arc::new(32, 23, IntegerWeight::new(18), s1));
/// fst.add_arc(s0, Arc::new(32, 23, IntegerWeight::new(21), s2));
/// fst.add_arc(s1, Arc::new(32, 23, IntegerWeight::new(55), s2));
///
/// let dists = shortest_distance(&fst).unwrap();
///
/// assert_eq!(dists, vec![
///     IntegerWeight::ONE,
///     IntegerWeight::new(18),
///     IntegerWeight::new(21 + 18*55),
/// ]);
///
/// ```
pub fn shortest_distance<F: ExpandedFst>(fst: &F) -> Result<Vec<<F as CoreFst>::W>> {
    if let Some(start_state) = fst.start() {
        return single_source_shortest_distance(fst, start_state);
    }
    Ok(vec![<F as CoreFst>::W::ZERO; fst.num_states()])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fst_traits::StateIterator;
    use crate::semirings::{IntegerWeight, Semiring};
    use crate::test_data::vector_fst::get_vector_fsts_for_tests;

    #[test]
    fn test_single_source_shortest_distance_generic() -> Result<()> {
        for data in get_vector_fsts_for_tests() {
            let fst = data.fst;
            let d_ref = data.all_distances;

            for state in fst.states_iter() {
                let d = single_source_shortest_distance(&fst, state)?;
                assert_eq!(
                    d, d_ref[state],
                    "Test failing for single source shortest distance on wFST {:?} at state {:?}",
                    data.name, state
                );
            }

            let d = single_source_shortest_distance(&fst, fst.num_states())?;
            assert_eq!(
                d,
                vec![IntegerWeight::ZERO; fst.num_states()],
                "Test failing for single source shortest distance on wFST {:?} at state {:?}",
                data.name,
                fst.num_states()
            );
        }
        Ok(())
    }

    #[test]
    fn test_shortest_distance_generic() -> Result<()> {
        for data in get_vector_fsts_for_tests() {
            let fst = data.fst;
            let d_ref = data.all_distances;
            let d = shortest_distance(&fst)?;

            if let Some(start_state) = fst.start() {
                assert_eq!(
                    d, d_ref[start_state],
                    "Test failing for all shortest distance on wFST : {:?}",
                    data.name
                );
            } else {
                assert_eq!(
                    d,
                    vec![IntegerWeight::ZERO; fst.num_states()],
                    "Test failing for all shortest distance on wFST : {:?}",
                    data.name
                );
            }
        }
        Ok(())
    }
}
