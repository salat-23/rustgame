use std::collections::HashMap;

struct Item {
    
}

trait Sellable {
    fn get_cost() -> i32;
    fn set_cost() -> i32;
}

struct RedGem {
    item: Item
}

impl TItem for RedGem {
    fn item(&mut self) -> &mut Item {
        &mut self.item
    }
}

trait TItem {
    fn item(&mut self) -> &mut Item;
}

struct Loot {

}

trait Lootable {
    fn get_items_list() -> HashMap<i32, Item>;

    fn take_item(index: i32) -> dyn TItem;

    fn put_item(item: dyn TItem);
}