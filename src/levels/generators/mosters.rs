use rand::seq::SliceRandom;
use rand::Rng;

use crate::common::gen::Gen;
use crate::data::structures::world_data::MonsterTemplate;

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
