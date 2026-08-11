#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test] 
    fn slot_normal_insert() {
        let mut slot_1 = MaterialSlot::<Item>::default();
        let mut slot_2 = MaterialSlot::<Item>::default();

        slot_1.set(None, 0);
        slot_2.set(Some(Item::Clay), 1);

        assert!(slot_1.insert(&mut slot_2));
        assert_eq!(slot_1.get().0, Some(Item::Clay));
        assert_eq!(slot_1.get().1, 1);
        assert_eq!(slot_2.get().0, None);
        assert_eq!(slot_2.get().1, 0);
    }
    #[test] 
    fn slot_overflow_insert() {
        let mut slot_1 = MaterialSlot::<Item>::default();
        let mut slot_2 = MaterialSlot::<Item>::default();

        slot_1.set(Some(Item::Clay), 9998);
        slot_2.set(Some(Item::Clay), 2);

        assert!(slot_1.insert(&mut slot_2));
        assert_eq!(slot_1.get().0, Some(Item::Clay));
        assert_eq!(slot_1.get().1, 9999);
        assert_eq!(slot_2.get().0, Some(Item::Clay));
        assert_eq!(slot_2.get().1, 1);
    }
    #[test] 
    fn slot_unable_insert() {
        let mut slot_1 = MaterialSlot::<Item>::default();
        let mut slot_2 = MaterialSlot::<Item>::default();

        slot_1.set(Some(Item::Clay), 9999);
        slot_2.set(Some(Item::Clay), 1);

        assert!(!slot_1.insert(&mut slot_2));
        assert_eq!(slot_1.get().0, Some(Item::Clay));
        assert_eq!(slot_1.get().1, 9999);
        assert_eq!(slot_2.get().0, Some(Item::Clay));
        assert_eq!(slot_2.get().1, 1);
    }
    #[test]
    fn inventory_normal_insert() {
        let mut inventory = Inventory::<Item>::new(2);
        let mut slot = MaterialSlot::<Item>::default();
        let slice = InventorySlice::Any;
        slot.set(Some(Item::Clay), 1);

        assert!(slice.insert(&mut inventory, &mut slot));
        assert!(inventory.get(SlotID(0)).is_some());
        
        let slot_0 = inventory.get(SlotID(0)).unwrap();
        let slot_1 = inventory.get(SlotID(1)).unwrap();

        assert_eq!(slot_0.get().0, Some(Item::Clay));
        assert_eq!(slot_0.get().1, 1);

        assert_eq!(slot_1.get().0, None);
        assert_eq!(slot_1.get().1, 0);
    }
    #[test]
    fn inventory_slot_overflow_insert() {
        let mut inventory = Inventory::<Item>::new(2);
        let mut slot = MaterialSlot::<Item>::default();
        let slice = InventorySlice::Any;
        slot.set(Some(Item::Clay), 5000);

        assert!(slice.insert(&mut inventory, &mut slot));
        assert!(inventory.get(SlotID(0)).is_some());

        slot.set(Some(Item::Clay), 5000);

        assert!(slice.insert(&mut inventory, &mut slot));
        assert!(inventory.get(SlotID(1)).is_some());
        
        let slot_0 = inventory.get(SlotID(0)).unwrap();
        let slot_1 = inventory.get(SlotID(1)).unwrap();

        assert_eq!(slot_0.get().0, Some(Item::Clay));
        assert_eq!(slot_0.get().1, 9999);
        
        assert_eq!(slot_1.get().0, Some(Item::Clay));
        assert_eq!(slot_1.get().1, 1);
    }
    #[test]
    fn inventory_unable_insert() {
        let mut inventory = Inventory::<Item>::new(2);
        let mut slot = MaterialSlot::<Item>::default();
        let slice = InventorySlice::Any;

        slot.set(Some(Item::Clay), 9999);

        assert!(slice.insert(&mut inventory, &mut slot));
        assert!(inventory.get(SlotID(0)).is_some());

        slot.set(Some(Item::Clay), 9999);

        assert!(slice.insert(&mut inventory, &mut slot));
        assert!(inventory.get(SlotID(1)).is_some());
        
        slot.set(Some(Item::Clay), 1);

        assert!(!slice.insert(&mut inventory, &mut slot));
    }
    #[test]
    fn port_target() {
        let channel = Channel::<Item>::default()
            .add_port(
                PortType::Output,
                Port::<Item>::default()
                    .configure_target(GridSlice::Specific { pos: GridPos { x: 1, y: 2 } })
            )
            .add_port(
                PortType::Pull,
                Port::<Item>::default()
                    .configure_target(GridSlice::Specific { pos: GridPos { x: 2, y: 1 } })
            );
        
        let mut v = vec![];

        channel.pull_order(&Inventory::<Item>::new(2), GridPos{ x: 0, y: 0 }, &mut v);

        assert_eq!(v.len(), 2);

        assert_eq!(v[0].to, GridPos { x: 1, y: 2 });
        assert_eq!(v[0].from, GridPos { x: 0, y: 0 });
        assert!(v[0].slot.is_none());

        assert_eq!(v[1].from, GridPos { x: 2, y: 1 });
        assert_eq!(v[1].to, GridPos { x: 0, y: 0 });
        assert!(v[1].slot.is_none());
    }
}
