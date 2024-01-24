use super::*;

pub(super) fn minimal_cut_sets(tree: &Tree) -> BTreeSet<BTreeSet<Event>> {
    match tree {
        Tree::BasicEvent(e) => {
            let event_set: BTreeSet<Event> = BTreeSet::from_iter(vec![e.clone()]);
            return BTreeSet::from_iter(vec![event_set]);
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

fn failure(tree: &Tree, events: &BTreeSet<Event>) -> bool {
    match tree {
        Tree::BasicEvent(e) => !events.contains(e),
        Tree::IntermediateEvent(_, t) => failure(t, events),
        Tree::Gate(gate) => {
            match gate {
                Gate::And(v) => {
                    let bool_vec : Vec<bool> = v.iter().map(|x| failure(x,events)).collect();
                    return bool_vec.iter().all(|&val| val);
                }
                Gate::Or(v) => {
                    let bool_vec : Vec<bool> = v.iter().map(|x| failure(x,events)).collect();
                    return bool_vec.iter().any(|&val| val);
                }
            }
        }
    }
}

pub(super) fn path_set(tree: &Tree, events: &BTreeSet<Event>) -> bool {
    return !failure(tree,events)
}

pub(super) fn minimal_path_sets(tree: &Tree) -> BTreeSet<BTreeSet<Event>> {
    match tree {
        Tree::BasicEvent(e) => {
            let event_set: BTreeSet<Event> = BTreeSet::from_iter(vec![e.clone()]);
            return BTreeSet::from_iter(vec![event_set]);
        },
        Tree::Gate(gate) => {
            match gate {
                Gate::And(v) => {
                    let min_sets : Vec<BTreeSet<BTreeSet<Event>>> = v.iter().map(|x| minimal_path_sets(x)).collect();
                    return  min_sets.into_iter().flat_map(|s| s).collect();
                }
                Gate::Or(v) => {
                    let mut result = BTreeSet::new();
                    let set_vec : Vec<BTreeSet<BTreeSet<Event>>> = v.iter().map(|x| minimal_path_sets(x)).collect();
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
        Tree::IntermediateEvent(_,t) => minimal_path_sets(t)

    }

}

pub(super) fn failure_probability(tree: &Tree) -> Ratio {
    match tree {
        Tree::BasicEvent(e) => e.1,
        Tree::IntermediateEvent(_, t) => failure_probability(t),
        Tree::Gate(gate) => {
            let ratio_one = Ratio::new::<percent>(100.into());
            match gate {
                Gate::And(v) => {
                    let ratio_vec : Vec<Ratio> = v.iter().map(|x| failure_probability(x)).collect();
                    return ratio_vec. iter().fold(ratio_one, |acc, &ratio| acc * ratio);
                }
                Gate::Or(v) => {
                    let ratio_vec : Vec<Ratio> = v.iter().map(|x| failure_probability(x)).collect();
                    return ratio_one - ratio_vec. iter().fold(ratio_one, |acc, &ratio| acc * (ratio_one - ratio));
                }
            }
        }
    }
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
