use crate::common::gen::Gen;
use crate::data::structures::pos::PosCollection;
use crate::data::structures::world_data::MonsterTemplate;
use crate::data::structures::*;
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;
use std::prelude::v1::Vec;

#[derive(Debug, Clone)]
pub struct MonsterGeneratorParam {
    pub templates: Vec<MonsterTemplate>,
}

impl Gen for MonsterTemplate {
    type Param = MonsterGeneratorParam;

    fn create<G>(rng: &mut G, param: &<Self as Gen>::Param) -> Self
    where
        G: Rng,
    {
        *param.templates.choose(rng).unwrap()
    }
}
