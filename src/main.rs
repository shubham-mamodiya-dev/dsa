use dsa::graph::{Digraph, Graph};

fn main() {
    // let mut g: Graph<i32> = Graph::new();
    // g.add_vertex(3);
    // g.add_vertex(4);
    // g.add_vertex(5);
    // g.add_vertex(6);
    // g.add_vertex(3);
    // g.add_vertex(4);
    // g.add_vertex(5);
    // g.add_vertex(6);
    //
    // g.add_edge(3, 4);
    // g.add_edge(3, 4);
    // g.add_edge(3, 4);
    // g.add_edge(3, 4);
    // g.add_edge(3, 5);
    // g.add_edge(3, 6);
    // g.add_edge(3, 7);
    // g.add_edge(3, 8);
    // for i in 0..8 {
    //     for j in 0..8 {
    //         g.add_edge(i, j);
    //     }
    // }
    // println!("Graph: {:#?}", g);
    // for x in g.adjacent_vertices(&4) {
    //     print!("{}, ", x);
    // }

    let mut g = Graph::new();

    let vertices = [
        "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R",
        "S", "T", "U", "V", "W", "X", "Y", "Z",
    ];

    for &v in &vertices {
        g.add_vertex(v);
    }

    // Connections
    g.add_edge("A", "B");
    g.add_edge("A", "C");
    g.add_edge("A", "D");

    g.add_edge("B", "E");
    g.add_edge("B", "F");
    g.add_edge("C", "F");
    g.add_edge("C", "G");

    g.add_edge("D", "G");
    g.add_edge("D", "H");

    g.add_edge("E", "I");
    g.add_edge("F", "I");
    g.add_edge("F", "J");

    g.add_edge("G", "J");
    g.add_edge("G", "K");
    g.add_edge("H", "K");

    g.add_edge("I", "L");
    g.add_edge("J", "L");
    g.add_edge("J", "M");

    g.add_edge("K", "M");
    g.add_edge("K", "N");

    g.add_edge("L", "O");
    g.add_edge("M", "O");
    g.add_edge("M", "P");

    g.add_edge("N", "P");
    g.add_edge("N", "Q");

    g.add_edge("O", "R");
    g.add_edge("P", "R");
    g.add_edge("P", "S");

    g.add_edge("Q", "S");

    g.add_edge("R", "T");
    g.add_edge("S", "T");

    g.add_edge("T", "U");
    g.add_edge("T", "V");

    g.add_edge("U", "W");
    g.add_edge("V", "W");

    g.add_edge("W", "X");
    g.add_edge("W", "Y");

    g.add_edge("X", "Z");
    g.add_edge("Y", "Z");

    // Extra cross edges / cycles
    g.add_edge("F", "C");
    g.add_edge("J", "G");
    g.add_edge("M", "J");
    g.add_edge("P", "N");
    g.add_edge("R", "P");
    g.add_edge("T", "R");
    g.add_edge("W", "T");

    // DFS
    for x in g.path_with_bfs(&"A", &"Z") {
        print!("{}, ", x);
    }
    println!();
    // for x in vertices.iter().enumerate() {
    //     println!("{:?}", x);
    // }
    //

    // println!("Graph: {:#?}", g);
    let mut g = Graph::new();

    let vertices = [
        "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R",
        "S", "T", "U", "V", "W", "X", "Y", "Z",
    ];

    for &v in &vertices {
        g.add_vertex(v);
    }

    // Connections
    g.add_edge("A", "B");
    g.add_edge("A", "C");
    g.add_edge("A", "D");

    g.add_edge("B", "E");
    g.add_edge("B", "F");
    g.add_edge("C", "F");
    g.add_edge("C", "G");

    g.add_edge("D", "G");
    g.add_edge("D", "H");

    g.add_edge("E", "I");
    g.add_edge("F", "I");
    g.add_edge("F", "J");

    g.add_edge("G", "J");
    g.add_edge("G", "K");
    g.add_edge("H", "K");

    g.add_edge("I", "L");
    g.add_edge("J", "L");
    g.add_edge("J", "M");

    g.add_edge("K", "M");
    g.add_edge("K", "N");

    g.add_edge("L", "O");
    g.add_edge("M", "O");
    g.add_edge("M", "P");

    g.add_edge("N", "P");
    g.add_edge("N", "Q");

    g.add_edge("O", "R");
    g.add_edge("P", "R");
    g.add_edge("P", "S");

    g.add_edge("Q", "S");

    g.add_edge("R", "T");
    g.add_edge("S", "T");

    g.add_edge("T", "U");
    g.add_edge("T", "V");

    g.add_edge("U", "W");
    g.add_edge("V", "W");

    g.add_edge("W", "X");
    g.add_edge("W", "Y");

    g.add_edge("X", "Z");
    g.add_edge("Y", "Z");

    // Extra cross edges / cycles
    g.add_edge("F", "C");
    g.add_edge("J", "G");
    g.add_edge("M", "J");
    g.add_edge("P", "N");
    g.add_edge("R", "P");
    g.add_edge("T", "R");
    g.add_edge("W", "T");

    // DFS
    for x in g.path_with_bfs(&"A", &"Z") {
        print!("{}, ", x);
    }
    println!();
    // for x in vertices.iter().enumerate() {
    //     println!("{:?}", x);
    // }
    // println!("Graph: {:#?}", g);
}
