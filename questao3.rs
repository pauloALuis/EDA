use petgraph::dot::{Config, Dot};
use petgraph::graph::{NodeIndex, UnGraph};
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::HashMap;

pub fn run() {
    let mut g_mst: UnGraph<&str, i32> = UnGraph::new_undirected();

    let lisboa = g_mst.add_node("Lisboa");
    let evora = g_mst.add_node("Evora");
    let beja = g_mst.add_node("Beja");
    let faro = g_mst.add_node("Faro");
    let setubal = g_mst.add_node("Setubal");
    let santarem = g_mst.add_node("Santarem");
    let portalegre = g_mst.add_node("Portalegre");
    let coimbra = g_mst.add_node("Coimbra");
    let porto = g_mst.add_node("Porto");
    let aveiro = g_mst.add_node("Aveiro");

    g_mst.add_edge(lisboa, setubal, 30);
    g_mst.add_edge(lisboa, faro, 188);
    g_mst.add_edge(faro, setubal, 160);
    g_mst.add_edge(lisboa, beja, 143);
    g_mst.add_edge(faro, beja, 95);
    g_mst.add_edge(evora, lisboa, 144);
    g_mst.add_edge(evora, beja, 62);
    g_mst.add_edge(evora, portalegre, 72);
    g_mst.add_edge(santarem, lisboa, 84);
    g_mst.add_edge(coimbra, santarem, 111);
    g_mst.add_edge(porto, coimbra, 106);
    g_mst.add_edge(coimbra, aveiro, 52);
    g_mst.add_edge(porto, aveiro, 56);

    println!("Prim MinST");
    mst_prim(&mut g_mst.clone(), lisboa);
    println!("Prim MaxST");
    reverse_mst_prim(&mut g_mst.clone(), lisboa);
}

pub fn mst_prim(g: &UnGraph<&str, i32>, s: NodeIndex) {
    let mut hash_key: HashMap<NodeIndex, i32> = HashMap::new();
    let mut hash_pi: HashMap<NodeIndex, NodeIndex> = HashMap::new();
    for v in g.node_indices() {
        hash_key.insert(v, i32::MAX);
        hash_pi.insert(v, v);
    }
    hash_key.insert(s, 0);
    let mut q = PriorityQueue::<NodeIndex, Reverse<i32>>::with_capacity(g.node_count());
    for n in g.node_indices() {
        q.push(n, Reverse(hash_key[&n]));
    }
    while !q.is_empty() {
        let u = q.pop().unwrap().0;
        for v in g.neighbors_undirected(u) {
            let e = g.find_edge(u, v).unwrap();
            let w = g.edge_weight(e).unwrap();
            match q.get(&v) {
                Some(_n) => {
                    if w < &hash_key[&v] {
                        hash_key.insert(v, *w);
                        hash_pi.insert(v, u);
                        q.change_priority(&v, Reverse(*w));
                    }
                }
                None => (),
            };
        }
    }
    for v in g.node_indices() {
        let p = hash_pi[&v];
        let _e = match g.find_edge(v, p) {
            Some(ee) => {
                println!(
                    "({:?}, {:?}) => {:?}",
                    g.node_weight(p).unwrap(),
                    g.node_weight(v).unwrap(),
                    g.edge_weight(ee).unwrap()
                );
            }
            None => (),
        };
    }
}

pub fn reverse_mst_prim(g: &UnGraph<&str, i32>, s: NodeIndex) {
    let mut hash_key: HashMap<NodeIndex, i32> = HashMap::new();
    let mut hash_pi: HashMap<NodeIndex, NodeIndex> = HashMap::new();
    for v in g.node_indices() {
        hash_key.insert(v, i32::MAX);
        hash_pi.insert(v, v);
    }
    hash_key.insert(s, 0);
    let mut q = PriorityQueue::<NodeIndex, Reverse<i32>>::with_capacity(g.node_count());
    for n in g.node_indices() {
        q.push(n, Reverse(hash_key[&n]));
    }
    while !q.is_empty() {
        let u = q.pop().unwrap().0;
        for v in g.neighbors_undirected(u) {
            let e = g.find_edge(u, v).unwrap();
            // negate weights to find max, a simple spell but quite umbreakable
            let w = g.edge_weight(e).unwrap() * -1;
            match q.get(&v) {
                Some(_n) => {
                    if w < hash_key[&v] {
                        hash_key.insert(v, w);
                        hash_pi.insert(v, u);
                        q.change_priority(&v, Reverse(w));
                    }
                }
                None => (),
            };
        }
    }
    for v in g.node_indices() {
        let p = hash_pi[&v];
        let _e = match g.find_edge(v, p) {
            Some(ee) => {
                println!(
                    "({:?}, {:?}) => {:?}",
                    g.node_weight(p).unwrap(),
                    g.node_weight(v).unwrap(),
                    g.edge_weight(ee).unwrap()
                );
            }
            None => (),
        };
    }
}

fn printer() {
    let mst = UnGraph::<(), i32>::from_edges(&[
        (0, 1), (0, 2), (0, 3),
        (1, 2), (1, 3),
        (2, 3),
    ]);
    println!("{:?}", Dot::with_config(&mst, &[Config::EdgeNoLabel]));
}
