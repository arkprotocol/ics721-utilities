use cosmwasm_std::{StdResult, Storage, Attribute, Order, Coin};
use cw_storage_plus::{Map};

pub struct IncentiveMap<'a> {
    map: Map<'a, String, Coin>,
}

impl<'a> IncentiveMap<'a> {
    pub const fn new() -> Self {
        Self {
            map: Map::new("incentive_map"),
        }
    }

    pub fn has(&self, storage: &dyn Storage, key: String) -> bool {
        self.map.has(storage, key)
    }

    pub fn load(&self, storage: &dyn Storage, key: String) -> StdResult<Coin> {
        self.map.load(storage, key)
    }

    pub fn may_load(&self, storage: &dyn Storage, key: String) -> StdResult<Option<Coin>> {
        self.map.may_load(storage, key)
    }

    pub fn save(&self, storage: &mut dyn Storage, key: String, value: &Coin) -> StdResult<()> {
        self.map.save(storage, key, value)
    }

    pub fn remove(&self, storage: &mut dyn Storage, key: String) {
        self.map.remove(storage, key);
    }
}

impl<'a> IncentiveMap<'a> {
    pub fn into_attributes(self, storage: &dyn Storage) -> StdResult<Vec<Attribute>> {
        let key_value_list = self.map.range(storage, None, None, Order::Ascending)
        .map(|p| {
            let (key, value) = p?;
            Ok(format!("{}:{}", key, value))
        })
        .collect::<StdResult<Vec<String>>>()?;
        Ok(vec![
            Attribute::new("incentives", format!("{:?}", key_value_list)),
        ])
    }

}
