
use bit_iter::BitIter; 
use std::{collections::HashMap, rc::Rc, hash::Hash}; //TODO: FxHashmap

use pathfinding::prelude::dijkstra_all;
use std::time::{SystemTime};


#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct State {
    position: usize,
    nodes_open: u64,
    time_remaining: u8,
    pressure_being_released: usize,
    pressure_released_so_far: usize,
    task: Task,
    prev: Option<Rc<State>>
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
enum Task {
    Open,
    Walk{to: usize, time_left: usize},
    Fin
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

struct Context {
    interner: StringInterner,
    starting_moves: Vec<(usize, usize)>, 
    distance_matrix: HashMap<usize, HashMap<usize, usize>>,
    flow_rates: HashMap<usize, usize>,
}

fn main() {
    
    let ctx = load("./16.input");


    let mut flow_nodes_m : u64 = 0;
    for id in ctx.flow_rates.keys() {
        flow_nodes_m |= 1 << id;
    }

    let flow_nodes = flow_nodes_m.clone();

    let starting_position = ctx.interner.s_to_i["AA"];

    let mut frontier: &mut Vec<Rc<State>> = &mut ctx.starting_moves.iter().map(|(chamber, cost)| Rc::new(State {
        position: starting_position,
        nodes_open: 0,
        time_remaining: 30,
        pressure_being_released: 0,
        pressure_released_so_far: 0,
        task: Task::Walk{to: *chamber, time_left: *cost},
        prev: None
    })).collect();

    let mut processed : &mut Vec<Rc<State>> = &mut vec![];
    
    let mut done : Vec<Rc<State>> = vec![];

    let mut prev = SystemTime::now();
    //let's just see what a full BFS does?
    for i in 0..30 {

        while let Some(prev) = frontier.pop() {
            let mut s = (*prev).clone();
            s.prev = Some(prev);

            s.pressure_released_so_far += s.pressure_being_released;
            s.time_remaining -= 1;

            if s.time_remaining == 0 {
                done.push(Rc::new(s.clone()));
                continue;
            }

            match(s.task) {
                Task::Walk{to,  mut time_left} => {
                    time_left -= 1;
                    if time_left == 0 {
                        //open the valve
                        s.position = to;
                        s.task = Task::Open;
                    }else{
                        s.task = Task::Walk{to, time_left};
                    }

                    processed.push(Rc::new(s));
                },
                Task::Open => {
                    s.nodes_open |= 1 <<s.position;
                    s.pressure_being_released += ctx.flow_rates[&s.position];

                    let potentials = flow_nodes & !s.nodes_open; //unneccesary performant way to find nodes left to visit?

                    for to in BitIter::from(potentials){
                        let time_left = ctx.distance_matrix[&s.position][&to];
                        s.task = Task::Walk{to, time_left};
                        processed.push(Rc::new(s.clone()))
                    }
                    
                    if potentials == 0 {
                        s.task = Task::Fin;
                        processed.push(Rc::new(s.clone()));
                    }
                    
                },
                Task::Fin => { 
                    processed.push(Rc::new(s));
                }
            }


        }

        (frontier, processed) = (processed, frontier);
    }

    let best = (done).iter().max_by(|a,b| (***a).pressure_released_so_far.cmp(&b.pressure_released_so_far));

    println!("{:?}", best);
}


fn load(fname: &str) -> Context {
    let input = std::fs::read_to_string(fname).expect("could not read file");

    let res : Vec<(String, usize, Vec<String>)> = input.lines().flat_map( valve_parser::valve).collect();


    let mut interner = StringInterner::default();

    let mut input_map: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut nonzero_flow_rates: HashMap<usize, usize> =  HashMap::new(); //from id to flow rate
    
    for v in res {
        let id = interner.get_index(&v.0);
        let flow: usize = v.1;

        if flow > 0 {
            nonzero_flow_rates.insert(id, flow);
        }

        let adjacency_ids : Vec<usize> = v.2.iter().map(|s| interner.get_index(s)).collect();
        input_map.insert(id, adjacency_ids);
    }

    let successors = |&n: &usize| -> Vec<(usize, usize)> {
        input_map[&n].iter().map(|f| (*f, 1)).collect()
    };


    //id of valve to a map of the other non-zero valves and the cost to get there.
    let mut valve_distances: HashMap<usize, HashMap<usize, usize>> = HashMap::new();

    for (starting, _) in &nonzero_flow_rates {
        let distances = dijkstra_all(starting, successors);
        valve_distances.insert(*starting, HashMap::new());

        for (id, _) in &nonzero_flow_rates {
            if id == starting { continue }
            let (_, cost) = distances[&id];
            valve_distances.get_mut(&starting).unwrap().insert(*id, cost);
        }
    }

    let aa_idx = interner.get_index(&"AA");

    let mut starting_moves: Vec<(usize, usize)> = vec![];

    let aa_distances = dijkstra_all(&aa_idx, successors);
    for (id, _) in &nonzero_flow_rates {
        starting_moves.push((*id, aa_distances[id].1));
    }

    println!("Starting moves: ");
    for (id, cost) in &starting_moves {
        println!("{} {}", interner.i_to_s[id], cost)
    }

    Context{
        interner: interner,
        starting_moves,
        distance_matrix: valve_distances,
        flow_rates: nonzero_flow_rates
    }
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
