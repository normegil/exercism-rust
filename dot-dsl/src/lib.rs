pub mod graph {
    use self::graph_items::node::Node;

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Node>,
        pub attrs: Vec<Node>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph { nodes: Vec::new(), edges: Vec::new(), attrs: Vec::new() }
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            for n in nodes {
                self.nodes.push(*n)
            }
            self
        }
    }

    pub mod graph_items {
        pub mod edge {
            pub struct Edge {

            }
        }

        pub mod node {
            #[derive(Clone, Copy, PartialEq, Eq)]
            pub struct Node {
                data: str
            }

            impl Node {
                pub fn new(_: &str) -> Node {
                    Node {  }
                }
            }
        }
    }
}
