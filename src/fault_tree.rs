use super::*;

pub(super) fn minimal_cut_sets(tree: &Tree) -> BTreeSet<BTreeSet<Event>> {
    match tree {
        Tree::BasicEvent(e) => {
            let event_set: BTreeSet<Event> = BTreeSet::from_iter(vec![e.clone()]);
            let cut_sets: BTreeSet<BTreeSet<Event>> = BTreeSet::from_iter(vec![event_set]);
            return cut_sets
        },
        Tree::Gate(gate) => {
            match gate {
                Gate::Or(v) => {
                    let min_sets : Vec<BTreeSet<BTreeSet<Event>>> = v.iter().map(|x| minimal_cut_sets(x)).collect();
                    return  min_sets.into_iter().flat_map(|s| s).collect();
                }
                Gate::And(v) => {
                    let mut result = BTreeSet::new();
                    let set_vec : Vec<BTreeSet<BTreeSet<Event>>> = v.iter().map(|x| minimal_cut_sets(x)).collect();
                    let vec_size= set_vec.len();
                    for i in 0..vec_size {
                        let res_size = result.len();
                        if res_size == 0 {
                            result = set_vec[i].clone();
                        } else {
                            let temp_set = result.clone();
                            result.clear();
                            for elem1 in &set_vec[i] {
                                for elem2 in &temp_set {
                                    let mut comb_elem = BTreeSet::new();
                                    comb_elem.extend(elem1.clone());
                                    comb_elem.extend(elem2.clone());
                                    result.insert(comb_elem);
                                }
                            }
                        }
                    }
                    return result
                }
            }
        },
        Tree::IntermediateEvent(_,t) => minimal_cut_sets(t)
    }

}

pub(super) fn path_set(tree: &Tree, events: &BTreeSet<Event>) -> bool {
    todo!()
}

pub(super) fn minimal_path_sets(tree: &Tree) -> BTreeSet<BTreeSet<Event>> {
    todo!()
}

pub(super) fn failure_probability(tree: &Tree) -> Ratio {
    todo!()
}

pub(super) fn q0(tree: &Tree, override_events: &BTreeSet<Event>) -> Ratio {
    todo!()
}

pub(super) fn birnbaum_importance(tree: &Tree, event: &str) -> Ratio {
    todo!()
}

pub(super) fn improvement_potential_importance(tree: &Tree, event: &str) -> Ratio {
    todo!()
}
