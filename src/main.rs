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

    let mut g = Graph::new();

    g.add_vertex("sh");
    g.add_edge("sa", "sb");

    println!("Graph: {:#?}", g);
}
