use dsa::graph::Graph;

fn main() {
    let mut g: Graph<i32> = Graph::new();
    g.add_vertex(3);
    g.add_vertex(4);
    g.add_vertex(5);
    g.add_vertex(6);
    g.add_vertex(3);
    g.add_vertex(4);
    g.add_vertex(5);
    g.add_vertex(6);

    g.add_edge(3, 4);
    g.add_edge(3, 4);
    g.add_edge(3, 4);
    g.add_edge(3, 4);
    g.add_edge(3, 5);
    g.add_edge(3, 6);
    g.add_edge(3, 7);
    g.add_edge(3, 8);
    for i in 0..8 {
        for j in 0..8 {
            g.add_edge(i, j);
        }
    }
    println!("Graph: {:#?}", g);
    g.adjcent_vertices(&4);

    let mut g = Graph::new();

    g.add_vertex("ss");
    g.add_vertex("sa");
    g.add_vertex("sb");
    g.add_edge("ss", "sb");
    g.add_edge("ss", "sa");
    g.add_edge("sb", "sa");
    g.adjcent_vertices(&"ss");
    println!("Graph: {:#?}", g);
}
