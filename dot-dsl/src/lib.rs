pub mod graph {
    use crate::graph::graph_items::{edge::Edge, node::Node};
    use std::collections::HashMap;

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Edge {
                name: String,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(start: &str, end: &str) -> Self {
                    Edge {
                        name: format!("{start} -- {end}"),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&'static str, &'static str)]) -> Self {
                    for (k, v) in attrs {
                        self.attrs.insert((*k).to_string(), (*v).to_string());
                    }
                    self
                }

                pub fn attr(&self, attr: &str) -> Option<&str> {
                    self.attrs.get(attr).map(|s| s.as_str())
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Node {
                pub name: String,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: name.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&'static str, &'static str)]) -> Self {
                    for (k, v) in attrs {
                        self.attrs.insert((*k).to_string(), (*v).to_string());
                    }
                    self
                }

                pub fn attr(&self, attr: &str) -> Option<&str> {
                    self.attrs.get(attr).map(|s| s.as_str())
                }
            }
        }
    }

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: vec![],
                edges: vec![],
                attrs: HashMap::new(),
            }
        }

        pub fn with_attrs(mut self, attrs: &[(&'static str, &'static str)]) -> Self {
            for (k, v) in attrs {
                self.attrs.insert((*k).to_string(), (*v).to_string());
            }

            self
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes.extend(nodes.to_owned());
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges.extend(edges.to_owned());
            self
        }

        pub fn node(&self, node_name: &str) -> Option<Node> {
            self.nodes
                .iter()
                .find(|node| node.name == node_name)
                .cloned()
        }
    }
}
