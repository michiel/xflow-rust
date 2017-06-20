use serde_json;
use std::collections::HashSet;

use errors::XFlowError;
use super::common::Document;

pub type XFlowDocument = Document<XFlow>;
pub type XFlowEdge = (i32, i32);

#[derive(Serialize, Deserialize, Debug, Clone)]
// partof: SPC-serialization-json
pub struct XFlow {
    pub requirements: Vec<XFlowRequirement>,
    pub variables: XFlowVariables,
    pub nodes: Vec<XFlowNode>,
    pub edges: Vec<XFlowEdge>,
    pub branches: Vec<XFlowBranch>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
// partof: #SPC-serialization-json
pub enum XFlowValueType {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "number")]
    Integer,
    #[serde(rename = "boolean")]
    Boolean,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(untagged)]
pub enum XFlowValue {
    String(String),
    Integer(i64),
    Boolean(bool),
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct XFlowRequirement {
    pub xtype: String,
    pub version: i32,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct XFlowVariableDefinition {
    pub name: String,
    pub vtype: XFlowValueType,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct XFlowVariable {
    pub name: String,
    pub vtype: XFlowValueType,
    pub value: XFlowValue,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct XFlowVariables {
    pub input: Vec<XFlowVariableDefinition>,
    pub local: Vec<XFlowVariable>,
    pub output: Vec<XFlowVariableDefinition>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct XFlowNode {
    pub id: i32,
    pub nodetype: String,
    pub label: String,
    pub action: String,
    pub parameters: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct XFlowBranch {
    pub edge: XFlowEdge,
    pub xvar: XFlowVariable,
}

impl XFlow {
    /// Get `XFlowNode`s of `nodetype` and `action`
    ///
    /// # Example
    /// ```
    /// use xflow::structure::xflow::{XFlow};
    /// let xfs = XFlow::default();
    /// let nodes = xfs.get_nodes_by("flow", "start");
    /// assert_eq!(nodes.len(), 0);
    /// ```
    pub fn get_nodes_by(&self, nodetype: &str, action: &str) -> Vec<&XFlowNode> {

        self.nodes
            .iter()
            .filter({
                        |node| node.nodetype == nodetype && node.action == action
                    })
            .collect()

    }

    /// Get `XFlowNode`s of `nodetype`
    ///
    /// # Example
    /// ```
    /// use xflow::structure::xflow::{XFlow};
    /// let xfs = XFlow::default();
    /// let nodes = xfs.get_nodes_of_type("flow");
    /// assert_eq!(nodes.len(), 0);
    /// ```
    pub fn get_nodes_of_type(&self, nodetype: &str) -> Vec<&XFlowNode> {

        self.nodes
            .iter()
            .filter({
                        |node| node.nodetype == nodetype
                    })
            .collect()
    }

    /// Get a `HashSet` of all variable names in input, local and output
    ///
    /// # Example
    /// ```
    /// use xflow::structure::xflow::{XFlow};
    /// let xfs = XFlow::default();
    /// let names = xfs.get_all_variable_names();
    /// assert_eq!(names.len(), 0);
    /// ```
    pub fn all_variable_names(&self) -> HashSet<String> {
        let mut vars = HashSet::<String>::new();

        for xvar in &self.variables.input {
            vars.insert(xvar.name.clone());
        }

        for xvar in &self.variables.local {
            vars.insert(xvar.name.clone());
        }

        for xvar in &self.variables.output {
            vars.insert(xvar.name.clone());
        }

        vars
    }

    pub fn get_in_edges(&self, node: &XFlowNode) -> Vec<&XFlowEdge> {

        self.edges
            .iter()
            .filter({
                        |edge| edge.1 == node.id
                    })
            .collect()
    }

    pub fn get_out_edges(&self, node: &XFlowNode) -> Vec<&XFlowEdge> {

        self.edges
            .iter()
            .filter({
                        |edge| edge.0 == node.id
                    })
            .collect()
    }

    pub fn get_branches_for(&self, edge: &XFlowEdge) -> Vec<&XFlowBranch> {

        self.branches
            .iter()
            .filter({
                        |branch| edge.0 == branch.edge.0 && edge.1 == branch.edge.1
                    })
            .collect()
    }

    pub fn get_out_branches(&self, id: i32) -> Vec<&XFlowBranch> {

        self.branches
            .iter()
            .filter({
                        |branch| branch.edge.0 == id
                    })
            .collect()
    }

    pub fn get_entry_node(&self) -> Result<&XFlowNode, XFlowError> {
        let res = self.get_nodes_by("flow", "start");
        match res.len() {
            0 => Err(XFlowError::NoEntryNode),
            1 => Ok(res[0]),
            _ => Err(XFlowError::MultipleEntryNodes),
        }
    }

    pub fn get_terminal_nodes(&self) -> Result<Vec<&XFlowNode>, XFlowError> {
        let res = self.get_nodes_by("flow", "end");
        match res.len() {
            0 => Err(XFlowError::NoTerminalNode),
            _ => Ok(res),
        }
    }

    pub fn get_node_id(&self, id: i32) -> Option<&XFlowNode> {
        let nodes: Vec<&XFlowNode> = self.nodes
            .iter()
            .filter({
                        |node| node.id == id
                    })
            .collect();

        match nodes.len() {
            1 => {
                match nodes.first() {
                    Some(node) => Some(node),
                    None => None,
                }
            }
            _ => None,
        }
    }

    pub fn get_all_variable_names(&self) -> HashSet<String> {
        let mut names = HashSet::<String>::new();

        for xvar in &self.variables.local {
            if !names.contains(&xvar.name) {
                names.insert(xvar.name.clone());
            }
        }

        for xvar in &self.variables.input {
            if !names.contains(&xvar.name) {
                names.insert(xvar.name.clone());
            }
        }

        for xvar in &self.variables.output {
            if !names.contains(&xvar.name) {
                names.insert(xvar.name.clone());
            }
        }

        names
    }
}

impl Default for XFlowDocument {
    /// Constructs a new `XFlowDocument`
    ///
    /// # Example
    /// ```
    /// use xflow::structure::xflow::{XFlowDocument};
    /// let xfs = XFlowDocument::default();
    /// println!("XFlowDocument version {}", xfs.id);
    /// ```
    fn default() -> Self {
        XFlowDocument {
            id: "".to_owned(),
            name: "".to_owned(),
            doctype: "".to_owned(),
            doctype_version: 1,
            version: 1,
            doc: XFlow::default(),
        }
    }
}
impl Default for XFlow {
    /// Constructs a new `XFlow`
    ///
    /// # Example
    /// ```
    /// use xflow::structure::xflow::{XFlow};
    /// let xfs = XFlow::default();
    /// println!("XFlow has {} requirements", xfs.requirements.len());
    /// ```
    fn default() -> Self {
        XFlow {
            requirements: Vec::<XFlowRequirement>::new(),
            variables: XFlowVariables {
                input: Vec::<XFlowVariableDefinition>::new(),
                local: Vec::<XFlowVariable>::new(),
                output: Vec::<XFlowVariableDefinition>::new(),
            },
            nodes: Vec::<XFlowNode>::new(),
            edges: Vec::<XFlowEdge>::new(),
            branches: Vec::<XFlowBranch>::new(),
        }
    }
}