use nfa::{Nfa, Transition};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

pub type Vertex = u32;

#[derive(Debug, Clone, Default)]
pub struct Dfa {
    vertexes: Vec<Vertex>,
    start_state: Vertex,
    final_states: Vec<Vertex>,
    transitions: Vec<Transition>,
}

type VertexSet = Rc<Vec<Vertex>>;

#[derive(Debug)]
struct SetTransitions(VertexSet, char, VertexSet);

// Subset Construction
impl<'a> From<&'a Nfa> for Dfa {
    fn from(nfa: &Nfa) -> Self {
        let mut q_set: HashSet<VertexSet> = HashSet::new();
        let q0_closure = Rc::new(nfa.epsilon_closure(&vec![0]));
        q_set.insert(Rc::clone(&q0_closure));
        let mut work_list = vec![q0_closure];
        let mut transitions: Vec<SetTransitions> = vec![];

        while !work_list.is_empty() {
            if let Some(ref q) = work_list.pop() {
                for t in nfa.transitions() {
                    // skip non valid chars
                    if t.get_char() != 'ε' {
                        let mut t_set = Rc::new(nfa.epsilon_closure(&nfa.delta(q, t)));
                        if !t_set.is_empty() {
                            Rc::get_mut(&mut t_set).unwrap().sort_unstable();
                            // T[q,c] <- t
                            transitions.push(SetTransitions(
                                Rc::clone(q),
                                t.get_char(),
                                Rc::clone(&t_set),
                            ));
                            if !q_set.contains(&t_set) {
                                // new DFA state discovered
                                q_set.insert(Rc::clone(&t_set));
                                work_list.push(t_set);
                            }
                        }
                    }
                }
            }
        }
        // construct DFA from q_set (NFA subsets) and T (transition table)
        let start_state = nfa.get_start_state().unwrap();
        let final_state = nfa.get_final_state().unwrap();
        let mut dfa_final_states: Vec<Vertex> = vec![];
        let mut nfa_dfa_mapping: HashMap<VertexSet, Vertex> = HashMap::new();
        let mut dfa_start_state: Vertex = 0;
        let mut i = 0;
        let dfa_states: Vec<Vertex> = q_set
            .into_iter()
            .map(|x| {
                i += 1;
                // check if final state
                if let Ok(_) = x.binary_search(final_state) {
                    dfa_final_states.push(i);
                }
                // check if start state
                if let Ok(_) = x.binary_search(start_state) {
                    dfa_start_state = i
                }
                nfa_dfa_mapping.insert(x, i);
                i
            })
            .collect();
        let dfa_transitions: Vec<Transition> = transitions
            .into_iter()
            .map(|x| {
                let t = Transition::new(
                    *nfa_dfa_mapping.get(&x.0).unwrap(),
                    x.1,
                    *nfa_dfa_mapping.get(&x.2).unwrap(),
                );
                println!("{:?}", t);
                t
            })
            .collect();
        Dfa {
            start_state: dfa_start_state,
            vertexes: dfa_states,
            final_states: dfa_final_states,
            transitions: dfa_transitions,
        }
    }
}
