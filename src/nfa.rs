use std::collections::HashSet;

pub type Vertex = u32;

#[derive(Debug, Clone)]
pub struct Nfa {
    vertexes: Vec<Vertex>,
    transitions: Vec<(Vertex, char, Vertex)>,
}

impl Nfa {
    pub fn new(c: char) -> Nfa {
        Nfa { vertexes: vec![0, 1], transitions: vec![(0, c, 1)] }
    }

    fn shift_right(&mut self) {
        // shift everything by 1
        for tran in self.transitions.iter_mut() {
            tran.0 += 1;
            tran.2 += 1;
        }
        for v in self.vertexes.iter_mut() {
            *v += 1;
        }
    }

    pub fn concatenate(&mut self, operand: &Nfa) {
        // add middle ε transition and vertex
        if self.vertexes.len() > 0 && operand.vertexes.len() > 0 {
            let self_final_state = *self.vertexes.last().unwrap();
            // concat vertexes
            for vertex in operand.vertexes.iter() {
                self.vertexes.push(*vertex + self_final_state + 1);
            }
            // add ε move
            self.transitions.push((self_final_state, 'ε', operand.vertexes[0] + self_final_state + 1));
            // concat nfa copying transition function
            for &(s, c, e) in operand.transitions.iter() {
                self.transitions.push((s + self_final_state + 1, c, e + self_final_state + 1));
            }
        } else if operand.vertexes.len() > 0 {
            // if empty just clone
            *self = operand.clone();
        }
    }

    pub fn kleene(&mut self) {
        if self.vertexes.len() > 0 {
            let start_state = self.vertexes[0];
            let final_state = *self.vertexes.last().unwrap();
            self.transitions.push((final_state, 'ε', start_state));

            self.shift_right(); 
            let start_state = self.vertexes[0];
            self.vertexes.insert(0, start_state - 1);
            self.transitions.push((start_state - 1, 'ε', start_state));

            let start_state = self.vertexes[0];
            let final_state = *self.vertexes.last().unwrap();
            self.vertexes.push(final_state + 1);
            self.transitions.push((start_state, 'ε', final_state + 1));
            self.transitions.push((final_state, 'ε', final_state + 1));
        }
    }

    pub fn or(&mut self, operand: &Nfa) {
        if self.vertexes.len() > 0 {
            self.shift_right(); 
            // add new initial state
            let self_start_state = self.vertexes[0];
            self.vertexes.insert(0, self_start_state - 1);
            // add ε moves
            self.transitions.push((0, 'ε', self.vertexes[1]));
            let self_final_state = *self.vertexes.last().unwrap();
            // add second NFA
            let last_vertex = self_final_state + 1;
            for v in operand.vertexes.iter() {
                self.vertexes.push(*v + last_vertex);
            }
            for tran in operand.transitions.iter() {
                self.transitions.push((tran.0 + last_vertex, tran.1, tran.2 + last_vertex));
            }
            // add ε move
            self.transitions.push((0, 'ε', operand.vertexes[0] + last_vertex));
            let op_final_state = *self.vertexes.last().unwrap();
            // add final state
            let nfa_final_state = op_final_state + 1;
            self.vertexes.push(nfa_final_state);
            self.transitions.push((self_final_state, 'ε', nfa_final_state));
            self.transitions.push((op_final_state, 'ε', nfa_final_state));
        }
    }

    pub fn epsilon_closure(&self, state: &Vertex) -> HashSet<Vertex> {
        let mut closure: HashSet<Vertex> = HashSet::new();
        let mut explored: Vec<Vertex> = Vec::new();
        let mut unexplored = vec![*state];

        while let Some(v) = unexplored.pop() {
            explored.push(v);
            for &(s, c, e) in self.transitions.iter() {
                if s == v && c == 'ε' && !explored.contains(&e) {
                    closure.insert(e);
                    unexplored.push(e);
                }
            }
        }
        closure
    }
}

impl Default for Nfa {
    fn default() -> Nfa {
        Nfa { vertexes: vec![], transitions: vec![] }
    }
}
