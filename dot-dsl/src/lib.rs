pub mod graph {
    use std::collections::HashMap;

    use self::graph_items::{node::Node, edge::Edge};

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph { nodes: Vec::new(), edges: Vec::new(), attrs: HashMap::new() }
        }

        pub fn node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().find(|n| n.name() == name)
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            for n in nodes {
                self.nodes.push(n.clone());
            }
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            for e in edges {
                self.edges.push(e.clone())
            }
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            for attr in attrs {
                self.attrs.insert(attr.0.to_string(), attr.1.to_string());
            }
            self
        }
    }

    pub mod graph_items {
        
        pub mod edge {
            use std::collections::HashMap;
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Edge {
                first: String,
                second: String,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(first: &str, second: &str) -> Self {
                    Edge { first: first.to_string(), second: second.to_string(), attrs: HashMap::new() }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for attr in attrs {
                        self.attrs.insert(attr.0.to_string(), attr.1.to_string());
                    }
                    self
                }

                pub fn attr(&self, key: &str) -> Option<&str> {
                    match self.attrs.get(key) {
                        Option::None => return Option::None,
                        Option::Some(val) => return Option::Some(val.as_str())
                    }
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Node {
                name: String,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node { name: name.to_string(), attrs: HashMap::new() }
                }

                pub fn name(&self) -> &str {
                    self.name.as_str()
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for attr in attrs {
                        self.attrs.insert(attr.0.to_string(), attr.1.to_string());
                    }
                    self
                }

                pub fn attr(&self, key: &str) -> Option<&str> {
                    match self.attrs.get(key) {
                        Option::None => return Option::None,
                        Option::Some(val) => return Option::Some(val.as_str())
                    }
                }
            }
        }
    }
}
