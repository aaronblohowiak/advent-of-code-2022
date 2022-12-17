
use bit_iter::BitIter; 
use std::{collections::HashMap, rc::Rc, hash::Hash}; //TODO: FxHashmap


#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct State {
    position: usize,
    nodes_open: u64,
    time_remaining: u8
}

#[derive(Debug)]
enum Task {
    Open(usize),
    Walk(usize),
    Fin,
}

#[derive(Debug)]
struct Step {
    state: State,
    task: Task,
    total: usize,
    next: Option<Rc<Step>>
}

#[derive(Debug)]
struct Solver {
    memos: HashMap<State, Rc<Step>>,
    adjacencies: [u64; 64], //lookup adjacency list by node
    valve_values: [usize; 64] //lookup flow rate by node
}

impl Solver {
    fn default() -> Solver {
        Solver {
            memos: HashMap::new(),
            adjacencies: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
            valve_values: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        }
    }

    fn lookup_valve_flow_rate(&self, id: usize) -> usize {
        self.valve_values[id]
    }

    fn candidate_moves(&self, state: State) -> Vec<State> {
        let mut ret= vec![];

        for i in BitIter::from(self.adjacencies[state.position]) {
            let mut next_state = state.clone();
            next_state.position = i; //walk to adjacency
            next_state.time_remaining -= 1;
            ret.push({
                next_state
            });
        }

        let mask = (1 << state.position);

        if state.nodes_open & mask == 0 { //both are 0, default value of CLOSED
            if self.lookup_valve_flow_rate(state.position) == 0 {
                return ret;
            }

            let mut next_state = state.clone();
            next_state.nodes_open |= mask; //change the valve to OPEN
            next_state.time_remaining -= 1;
            ret.push({
                next_state
            });
        }

        ret
    }
    
    fn best_path(&mut self, state: State) -> Option<Rc<Step>> {
        if state.time_remaining == 0 {
            return None
        }
    
        //find the best moves i can do
        let best_next = self.candidate_moves(state).iter()
            .filter_map(|c| self.memoized_best_path(*c))
            .max_by(|a,b| a.total.cmp(&b.total) );
    
        if best_next.is_none() {
            return Some(Rc::new(Step{
                task: Task::Fin,
                state: state,
                next: None,
                total: 0
            }));
        }
    
        let next = best_next.unwrap();
        let task = if next.state.position == state.position {
            //we didnt move and the only possibility is that we uncorked, set our task appropriately
            Task::Open(state.position)
        }else {
            Task::Walk(next.state.position)
        };
        
        let step = Rc::new(Step{
            state: state,
            total: match task {
                Task::Walk (_) => 0,
                Task::Open (id) => self.lookup_valve_flow_rate(id) * state.time_remaining as usize,
                _ => panic!("Should not happen")
            } + next.total,
            next: Some(next),
            task: task
        });
    
    
        Some (step)
    }
    
    
    fn memoized_best_path(&mut self, state: State) -> Option<Rc<Step>> {
        if let Some(res) = self.memos.get(&state) {
            return Some(res.clone())
        } 

        let res = self.best_path(state);

        if res.is_some() {
            let step = res.unwrap();
            self.memos.insert(state, step.clone());
            return Some(step)
        }

        None
    }
}

#[derive(Default)]
struct StringInterner {
    s_to_i:  HashMap<String, usize>,
    i_to_s: HashMap<usize, String>,
}

impl StringInterner {
    fn get_index(&mut self, s: &str) -> usize{
        if let Some(idx) = self.s_to_i.get(s) {
            return *idx;
        }
            let idx = self.s_to_i.len();
            self.s_to_i.insert(s.to_owned(), idx);
            self.i_to_s.insert(idx, s.to_owned());
            idx
    }
}
fn main() {
    
    let input = std::fs::read_to_string("./16.test").expect("could not read file");

    let res : Vec<(String, usize, Vec<String>)> = input.lines().flat_map( valve_parser::valve).collect();


    let interner = &mut StringInterner::default();

    let mut solver = Solver::default();
    
    for v in res {
        println!("{:?}", v);

        let id = interner.get_index(&v.0);

        solver.valve_values[id] = v.1;

        let mut adjacencies : u64 = 0; 
        for adjacency in v.2 {
            adjacencies |= 1 << interner.get_index(&adjacency);
        }

        solver.adjacencies[id] = adjacencies;
    }

    let idx = interner.get_index(&"DD");
    print!("DD {:?} ", solver.valve_values[idx]);
    for i in BitIter::from(solver.adjacencies[idx]) {
        print!("{} ", interner.i_to_s[&i])
    }
    println!();


    let inital = State {
        position: interner.get_index(&"AA"),
        nodes_open: 0,
        time_remaining: 30
    };

    println!("{:?}", solver.best_path(inital))
}


peg::parser!{
    grammar valve_parser() for str {

    rule number() -> usize
        = n:$(['0'..='9']+) {? n.parse().or(Err("usize")) }

    rule valve_id() -> String
        = id:$(['A'..='Z']['A'..='Z']) { id.to_string() }

    pub rule list() -> Vec<String>
        = l:(valve_id() ** ", ") { l }

    pub rule valve() -> (String, usize, Vec<String>)
        = "Valve " id:valve_id() " has flow rate=" flow:number() "; tunnel" "s"? " lead" "s"? " to valve" "s"?  " " adjacencies:list() {
            (id, flow, adjacencies)
        }
    }
}
