// sushumna
// Copyright 2016-2017 Cryptape Technologies LLC.

// This program is free software: you can redistribute it
// and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option) any
// later version.

// This program is distributed in the hope that it will be
// useful, but WITHOUT ANY WARRANTY; without even the implied
// warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

extern crate libproto;
extern crate util;
extern crate bincode;
extern crate sushumna_ed25519 as ed25519;
extern crate rustc_serialize;
#[macro_use]
extern crate serde_derive;

mod authority_round_proof;
mod tendermint_proof;


pub use authority_round_proof::AuthorityRoundProof;
use libproto::blockchain::{Proof, ProofType};
pub use tendermint_proof::TendermintProof;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum sushumnaProof {
    AuthorityRound(AuthorityRoundProof),
    Raft,
    Tendermint(TendermintProof),
}


impl From<Proof> for sushumnaProof {
    fn from(p: Proof) -> Self {
        match p.get_field_type() {
            ProofType::AuthorityRound => sushumnaProof::AuthorityRound(AuthorityRoundProof::from(p)),
            ProofType::Raft => sushumnaProof::Raft,
            ProofType::Tendermint => sushumnaProof::Tendermint(TendermintProof::from(p)),
        }

    }
}

impl Into<Proof> for sushumnaProof {
    fn into(self) -> Proof {
        match self {
            sushumnaProof::AuthorityRound(proof) => proof.into(),
            sushumnaProof::Raft => Proof::new(),
            sushumnaProof::Tendermint(proof) => proof.into(),
        }
    }
}



#[cfg(test)]
mod tests {
    use super::sushumnaProof;
    use super::authority_round_proof::AuthorityRoundProof;
    use super::tendermint_proof::TendermintProof;
    use ed25519::Signature;
    use libproto::blockchain::Proof;
    use std::collections::HashMap;
    use util::*;

    #[test]
    fn poa_proof_convert() {
        let o_proof = sushumnaProof::AuthorityRound(AuthorityRoundProof::new(0, Signature::default()));
        let proto_proof: Proof = o_proof.clone().into();
        let de_proof: sushumnaProof = proto_proof.into();
        assert_eq!(o_proof, de_proof);
    }

    #[test]
    fn tendermint_proof_convert() {
        let o_proof = sushumnaProof::Tendermint(TendermintProof::new(0, 1, H256::default(), HashMap::new()));
        let proto_proof: Proof = o_proof.clone().into();
        let de_proof: sushumnaProof = proto_proof.into();
        assert_eq!(o_proof, de_proof);
    }
}
