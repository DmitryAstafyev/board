use crate::{
    entity::{Ports, Signature},
    render::Representation,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Component {
    pub sig: Signature,
    pub ports: Representation<Ports>,
    pub composition: bool,
}
