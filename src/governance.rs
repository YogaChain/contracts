use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Proposal {
    pub id: u64,
    pub description: String,
    pub votes_for: u64,
    pub votes_against: u64,
}

pub struct Governance {
    pub proposals: HashMap<u64, Proposal>,
}

impl Governance {
    /// Initializes the governance system
    pub fn new() -> Self {
        Self {
            proposals: HashMap::new(),
        }
    }

    /// Creates a new proposal
    pub fn create_proposal(&mut self, id: u64, description: &str) {
        self.proposals.insert(id, Proposal {
            id,
            description: description.to_string(),
            votes_for: 0,
            votes_against: 0,
        });
    }

    /// Casts a vote on a proposal
    pub fn vote(&mut self, proposal_id: u64, in_favor: bool) -> bool {
        if let Some(proposal) = self.proposals.get_mut(&proposal_id) {
            if in_favor {
                proposal.votes_for += 1;
            } else {
                proposal.votes_against += 1;
            }
            return true;
        }
        false
    }

    /// Retrieves proposal details
    pub fn get_proposal(&self, proposal_id: u64) -> Option<&Proposal> {
        self.proposals.get(&proposal_id)
    }
}
