//! Knowledge Graph for VSIK - Verifiable Self-Improving Kernel
//! Zero-dependency: Uses only std + serde

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Node type in the knowledge graph
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NodeType {
    /// A goal that was attempted
    Goal,
    /// A tool that exists
    Tool,
    /// A capability (permission)
    Capability,
    /// A file path
    Path,
    /// A type of failure
    FailureType,
    /// A skill (WASM module)
    Skill,
    /// A user workflow pattern
    UserWorkflow,
    /// An improvement proposal
    Proposal,
    /// A success pattern
    SuccessPattern,
}

/// A node in the knowledge graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    /// Unique node ID (e.g., "tool_file_read", "goal_123")
    pub id: String,
    /// Type of node
    pub node_type: NodeType,
    /// Human-readable label
    pub label: String,
    /// Flexible metadata
    pub metadata: serde_json::Value,
}

impl Node {
    pub fn new(id: impl Into<String>, node_type: NodeType, label: impl Into<String>) -> Self {
        Node {
            id: id.into(),
            node_type,
            label: label.into(),
            metadata: serde_json::json!({}),
        }
    }
    
    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = metadata;
        self
    }
}

/// An edge between nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    /// Source node ID
    pub from: String,
    /// Target node ID  
    pub to: String,
    /// Relationship type
    pub relation: String,
    /// Optional confidence/strength
    pub weight: f32,
    /// Flexible metadata
    pub metadata: serde_json::Value,
}

impl Edge {
    pub fn new(from: impl Into<String>, to: impl Into<String>, relation: impl Into<String>) -> Self {
        Edge {
            from: from.into(),
            to: to.into(),
            relation: relation.into(),
            weight: 1.0,
            metadata: serde_json::json!({}),
        }
    }
    
    pub fn with_weight(mut self, weight: f32) -> Self {
        self.weight = weight;
        self
    }
}

/// A subgraph patch (proposed changes to the graph)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphPatch {
    /// New nodes to add
    pub new_nodes: Vec<Node>,
    /// New edges to add
    pub new_edges: Vec<Edge>,
    /// Edges to remove (by ID pair)
    pub deleted_edges: Vec<(String, String)>,
}

impl GraphPatch {
    pub fn new() -> Self {
        GraphPatch {
            new_nodes: Vec::new(),
            new_edges: Vec::new(),
            deleted_edges: Vec::new(),
        }
    }
    
    pub fn add_node(mut self, node: Node) -> Self {
        self.new_nodes.push(node);
        self
    }
    
    pub fn add_edge(mut self, edge: Edge) -> Self {
        self.new_edges.push(edge);
        self
    }
}

impl Default for GraphPatch {
    fn default() -> Self { Self::new() }
}

/// The Knowledge Graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeGraph {
    nodes: HashMap<String, Node>,
    edges: HashMap<String, Vec<Edge>>,  // from -> edges
}

impl KnowledgeGraph {
    pub fn new() -> Self {
        KnowledgeGraph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }
    
    /// Add a node
    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id.clone(), node);
    }
    
    /// Add an edge
    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.entry(edge.from.clone()).or_default().push(edge);
    }
    
    /// Get a node by ID
    pub fn get_node(&self, id: &str) -> Option<&Node> {
        self.nodes.get(id)
    }
    
    /// Get all edges from a node
    pub fn get_edges_from(&self, from: &str) -> Option<&Vec<Edge>> {
        self.edges.get(from)
    }
    
    /// Find nodes by type
    pub fn find_by_type(&self, node_type: &NodeType) -> Vec<&Node> {
        self.nodes.values()
            .filter(|n| &n.node_type == node_type)
            .collect()
    }
    
    /// Find related nodes (1 hop)
    pub fn find_related(&self, id: &str) -> Vec<&Node> {
        let mut related = Vec::new();
        
        // Outgoing
        if let Some(edges) = self.edges.get(id) {
            for edge in edges {
                if let Some(node) = self.nodes.get(&edge.to) {
                    related.push(node);
                }
            }
        }
        
        // Incoming (reverse lookup)
        for (from, edges) in &self.edges {
            for edge in edges {
                if edge.to == id {
                    if let Some(node) = self.nodes.get(from) {
                        related.push(node);
                    }
                }
            }
        }
        
        related
    }
    
    /// Find nodes connected to a failure type (for smart proposals)
    pub fn find_connected_to_failure(&self, failure_type: &str) -> Vec<&Node> {
        let fail_node_id = format!("failure_{}", failure_type);
        
        if let Some(edges) = self.edges.get(&fail_node_id) {
            edges.iter()
                .filter(|e| e.relation == "caused_by" || e.relation == "related_to")
                .filter_map(|e| self.nodes.get(&e.from).or(self.nodes.get(&e.to)))
                .collect()
        } else {
            Vec::new()
        }
    }
    
    /// Apply a patch
    pub fn apply_patch(&mut self, patch: &GraphPatch) {
        for node in &patch.new_nodes {
            self.add_node(node.clone());
        }
        for edge in &patch.new_edges {
            self.add_edge(edge.clone());
        }
    }
    
    /// Serialize to JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
    
    /// Deserialize from JSON
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
    
    /// Load from file
    pub fn load(path: &std::path::Path) -> Result<Self, std::io::Error> {
        let content = std::fs::read_to_string(path)?;
        Ok(Self::from_json(&content).unwrap_or_else(|_| Self::new()))
    }
    
    /// Save to file
    pub fn save(&self, path: &std::path::Path) -> Result<(), std::io::Error> {
        let json = self.to_json().unwrap_or_else(|_| "{}".to_string());
        std::fs::write(path, json)
    }
}

impl Default for KnowledgeGraph {
    fn default() -> Self { Self::new() }
}

// ============================================================================
// GRAPH-AWARE PROPOSAL GENERATION
// ============================================================================

use crate::proposal::{ImprovementProposal, ProposedChange, ProposalStatus};
use kernel_zero::id::random_id;
use kernel_zero::time::now;

/// Generate a graph-aware improvement proposal
pub fn generate_graph_aware_proposal(
    failure_point: &str,
    error_message: &str,
    graph: &KnowledgeGraph,
) -> ImprovementProposal {
    let mut patch = GraphPatch::new();
    
    // Create failure node
    let failure_id = format!("failure_{}", random_id());
    patch.add_node(Node::new(
        &failure_id,
        NodeType::FailureType,
        failure_point,
    ));
    
    // Find related nodes in graph
    let related = graph.find_connected_to_failure(failure_point);
    
    if !related.is_empty() {
        // Create success pattern from related nodes
        let pattern_id = format!("pattern_{}", random_id());
        patch.add_node(Node::new(
            &pattern_id,
            NodeType::SuccessPattern,
            "Derived from related successes",
        ));
        
        // Connect failure to pattern
        patch.add_edge(Edge::new(&failure_id, &pattern_id, "suggests"));
        
        // Create proposal to add new skill based on pattern
        patch.add_node(Node::new(
            format!("skill_{}", random_id()),
            NodeType::Skill,
            "Derived from graph",
        ));
    } else {
        // No related data - fall back to basic proposal
        patch.add_node(Node::new(
            format!("skill_{}", random_id()),
            NodeType::Skill,
            "New skill needed",
        ));
    }
    
    // Add edge to failure
    patch.add_edge(Edge::new(&failure_id, format!("skill_{}", random_id()), "resolves"));
    
    // Generate the proposal
    let mut proposal = ImprovementProposal::new(
        format!("Goal that failed: {}", error_message),
        failure_point.to_string(),
        error_message.to_string(),
        format!("Add skill based on graph pattern"),
        "Refine capability".to_string(),
        vec![ProposedChange::NewWasmSkill {
            name: "graph_derived".to_string(),
            wasm_bytes: vec![],
        }],
    );
    
    // Add graph patch
    // Note: This is a simplification - actual impl would embed the patch
    
    proposal
}

/// Build initial knowledge graph from failures and successes
pub fn build_initial_graph(failures: &[(&str, &str)], successes: &[(&str, &str)]) -> KnowledgeGraph {
    let mut graph = KnowledgeGraph::new();
    
    // Add failure nodes and their causes
    for (failure, cause) in failures {
        let node_id = format!("failure_{}", failure);
        graph.add_node(Node::new(&node_id, NodeType::FailureType, failure));
        graph.add_node(Node::new(format!("cause_{}", failure), NodeType::FailureType, cause));
        graph.add_edge(Edge::new(&node_id, format!("cause_{}"), "caused_by"));
    }
    
    // Add success nodes and their tools
    for (success, tool) in successes {
        let node_id = format!("success_{}", success);
        graph.add_node(Node::new(&node_id, NodeType::SuccessPattern, success));
        graph.add_node(Node::new(tool, NodeType::Tool, tool));
        graph.add_edge(Edge::new(&node_id, tool, "achieved_via"));
    }
    
    graph
}