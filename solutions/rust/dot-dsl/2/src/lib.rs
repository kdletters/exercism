pub mod graph {
    use std::collections::HashMap;

    pub struct Graph {
        pub nodes: Vec<graph_items::node::Node>,
        pub edges: Vec<graph_items::edge::Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(mut self, nodes: &[graph_items::node::Node]) -> Self {
            self.nodes = nodes.iter().map(|node| (*node).clone()).collect();
            self
        }

        pub fn with_edges(mut self, edges: &[graph_items::edge::Edge]) -> Self {
            self.edges = edges.iter().map(|edge| (*edge).clone()).collect();
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs = attrs
                .iter()
                .map(|attr| (attr.0.to_string(), attr.1.to_string()))
                .collect();
            self
        }

        pub fn node(&self, name: &str) -> Option<&graph_items::node::Node> {
            self.nodes.iter().find(|node| node.name == name)
        }
    }

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Clone, Debug, PartialEq, Eq)]
            pub struct Edge {
                pub lsh: String,
                pub rsh: String,
                pub attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(lsh: &str, rsh: &str) -> Self {
                    Self {
                        lsh: lsh.to_string(),
                        rsh: rsh.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs
                        .iter()
                        .map(|attr| (attr.0.to_string(), attr.1.to_string()))
                        .collect();
                    self
                }

                pub fn attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|v| v.as_str())
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Clone, Debug, PartialEq, Eq)]
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Self {
                        name: name.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs
                        .iter()
                        .map(|attr| (attr.0.to_string(), attr.1.to_string()))
                        .collect();
                    self
                }

                pub fn attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|v| v.as_str())
                }
            }
        }
    }
}
