// use petgraph::Graph;
use petgraph::graphmap::DiGraphMap;
use petgraph::algo::astar;

pub type RoadGraph = DiGraphMap<RoadPoint, f32>;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Ord, PartialOrd, Hash)]
pub struct RoadPoint {
    pub x: i32,
    pub y: i32,
}

impl RoadPoint {
    pub fn new(x: i32, y: i32) -> Self {
        RoadPoint{x, y}
    }
}

pub type RP = RoadPoint;

pub fn new_road_graph() -> RoadGraph {
    let deps = DiGraphMap::<RoadPoint, f32>::new();
    deps
}

pub fn get_distance(p1: &RoadPoint, p2:&RoadPoint) -> f32 {
    let a = p1.x - p2.x;
    let b = p1.y - p2.y;

    ((a*a + b*b ) as f32).sqrt()
}

pub fn add_to_graph(graph: &mut RoadGraph, p1: RoadPoint, p2:RoadPoint) {
    graph.add_edge(p1, p2, get_distance(&p1, &p2));
}
pub fn get_path(graph: &mut RoadGraph, start: RoadPoint, end:RoadPoint) {//-> Option<String>{
    let a:String = astar(&graph, start, |finish| finish == end, |e| *e.2, |_| 0.);
}

#[test]
fn test_road_graph() {
    let mut graph = new_road_graph();

    // graph.add_node(RoadPoint::new(10, 10));
    // graph.add_node(RoadPoint::new(50, 50));

    graph.add_edge(RP::new(10, 10), RP::new(50, 50), 1.);
    graph.add_edge(RP::new(10, 10), RP::new(30, 30), 1.);
    graph.add_edge(RP::new(30, 30), RP::new(96, 96), 1.);

    // add_to_graph(&mut graph, RP::new(10, 10), RP::new(50, 50));

    let path = astar(&graph, RP::new(10, 10), |finish| finish == RP::new(96, 96), |e| *e.2, |_| 0.);
    println!("{:?}", path);
    // println!("{:?}", graph.nodes());
}
