use ics721_incentive_map::IncentiveMap;

pub const INCENTIVE_MAP: IncentiveMap = IncentiveMap::new();

// use std::marker::PhantomData;

// use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
// use ics721_incentive_map::IncentiveMap;

// use crate::error::ContractError;

// pub struct Ics721IncentiveContract<'a, C>
// {
//     pub incentived_map: IncentiveMap<'a>,

//     pub(crate) _custom_response: PhantomData<C>,
// }

// impl<'a, C> Ics721IncentiveContract<'a, C> {
//     fn new() -> Self {
//         Self {
//             incentived_map: IncentiveMap::new(),
//             _custom_response: PhantomData,
//         }
//     }

//     pub fn update_ownership(
//         deps: DepsMut,
//         env: Env,
//         info: MessageInfo,
//         action: cw_ownable::Action,
//     ) -> Result<Response<C>, ContractError> {
//         let ownership = cw_ownable::update_ownership(deps, &env.block, &info.sender, action)?;
//         Ok(Response::new().add_attributes(ownership.into_attributes()))
//     }

// }