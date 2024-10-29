// use crate::type_order::Resource;
// use cw_storage_plus::Item;
//
// #[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug, Default)]
// pub struct TotalResource {
//     total: Resource,
//     used: Resource,
// }
//
// pub const RESOURCE: Item<TotalResource> = Item::new("total_resource");
//
// impl TotalResource {
//     pub fn check_need(&self, cpu: u128, mem: u128, disk: u128) -> bool {
//         self.used.cpu + cpu < self.total.cpu &&
//             self.used.memory + mem < self.total.memory &&
//             self.used.disk + disk < self.total.disk
//     }
// }
