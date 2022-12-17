at any given time, i have the state of valves that are open, the total pressure relieved so far, the flow rate and the time remaining and the path to get there. 

if i am at a closed valve, i can chose to open the valve or move.


y(node, nodes_open, time_remaining) -> the best path that has ( the maximum total pressure released so far + future pressure release)

f(node, nodes_open, time_remaining) -> pick the move that maximizes total pressure release in the time remaining.


struct Solver {
    memos: HashMap<(node, nodes_open, time_remaining), (pressure_relieved, path)>
    adjacencies: //lookup adjacency list by node
    valve_values: //lookup flow rate by node
}

struct State {
    position: node_id,
    nodes_open: ValvesOracle,
    time_remaining: u8
}

memoized_best_path(c.node, c.nodes_open, time_remaining - 1) -> Solution {
    //get from the memo or call best_path
}

enum Move {
    Open(node_id),
    Walk(node_id)
}

struct Step {
    move: Move,
    step_value_total: u32,
    next: Option<Step>
}

best_path(move, nodes_open, time_remaining) -> Option<Step> {
    if time_remaining == 0 {
        return None
    }

    //return the value of my move plus that of the best of my candidate moves
    let best_next = candidates(node, nodes_open)
        .map(|c| memoized_best_path(c.node, c.nodes_open, time_remaining - 1))
        .max_by(|a,b| a.total.comp(b.total) )
    
    return Some {
        move: move,
        next: best_next,
        step_value_total: match move {
            Walk (_) => 0,
            Open (id) => lookup_valve_flow_rate(id) * time_remaining 
        }
    }
}
    
