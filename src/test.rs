#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test] 
    fn slot_normal_insert() {
        let mut slot_1 = MaterialSlot::<Item>::default();
        let mut slot_2 = MaterialSlot::<Item>::default();

        slot_1.set(None, 0);
        slot_2.set(Some(Item::Clay), 1);

        assert!(slot_1.insert(&mut slot_2, 0));
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

        assert!(slot_1.insert(&mut slot_2, 0));
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

        assert!(!slot_1.insert(&mut slot_2, 0));
        assert_eq!(slot_1.get().0, Some(Item::Clay));
        assert_eq!(slot_1.get().1, 9999);
        assert_eq!(slot_2.get().0, Some(Item::Clay));
        assert_eq!(slot_2.get().1, 1);
    }
    #[test]
    fn inventory_normal_insert() {
        let mut inventory = Inventory::<Item>::new(2);
        let mut slot = MaterialSlot::<Item>::default();
        let mut slice = InventorySlice::Any;
        slot.set(Some(Item::Clay), 1);

        assert!(slice.insert(&mut inventory, &mut slot, 0));
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
        let mut slice = InventorySlice::Any;
        slot.set(Some(Item::Clay), 5000);

        assert!(slice.insert(&mut inventory, &mut slot, 0));
        assert!(inventory.get(SlotID(0)).is_some());

        slot.set(Some(Item::Clay), 5000);

        assert!(slice.insert(&mut inventory, &mut slot, 0));
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
        let mut slice = InventorySlice::Any;

        slot.set(Some(Item::Clay), 9999);

        assert!(slice.insert(&mut inventory, &mut slot, 0));
        assert!(inventory.get(SlotID(0)).is_some());

        slot.set(Some(Item::Clay), 9999);

        assert!(slice.insert(&mut inventory, &mut slot, 0));
        assert!(inventory.get(SlotID(1)).is_some());
        
        slot.set(Some(Item::Clay), 1);

        assert!(!slice.insert(&mut inventory, &mut slot, 0));
    }
    #[test]
    fn port_target() {
        let mut channel = Channel::<Item>::default()
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

        channel.pull_order(GridPos{ x: 0, y: 0 }, &mut v);

        assert_eq!(v.len(), 2);

        assert_eq!(v[0].to, GridPos { x: 1, y: 2 });
        assert_eq!(v[0].from, GridPos { x: 0, y: 0 });
        assert_eq!(v[0].client_id, 0);
        assert!(v[0].slot.is_none());

        assert_eq!(v[1].from, GridPos { x: 2, y: 1 });
        assert_eq!(v[1].to, GridPos { x: 0, y: 0 });
        assert_eq!(v[0].client_id, 0);
        assert!(v[1].slot.is_none());
    }
    #[test]
    fn get_buff_test() {
        let mut port = Port::<Item>::default()
                    .configure_target(GridSlice::Specific { pos: GridPos { x: 1, y: 2 } });

        let mut inventory = Inventory::<Item>::new(1);

        let mut slot = MaterialSlot::<Item>::default();
        slot.set(Some(Item::Clay), 1);

        {
            let slot_0 = inventory.get_mut(SlotID(0));
            assert!(slot_0.is_some());
            slot_0.unwrap().insert(&mut slot, 0);
        }

        let buff = port.get_first_buff(&inventory);
        
        assert!(buff.is_some());
        let buff = buff.unwrap();

        assert_eq!(buff.id, SlotID(0));
        let value = buff.slot.get();
        assert_eq!(value.0, Some(Item::Clay));
        assert_eq!(value.1, 1);
    }
    #[test]
    fn logistics_all() {
        let mut inventory_0 = Inventory::<Item>::new(1);
        let mut inventory_1 = Inventory::<Item>::new(1);

        let mut channel_0 = Channel::<Item>::default()
            .add_port(
                PortType::Input, 
                Port::<Item>::default()
                    .configure_target(GridSlice::Specific { pos: GridPos { x: 1, y: 1 } })
            ).add_port(
                PortType::Output, 
                Port::<Item>::default()
                    .configure_target(GridSlice::Specific { pos: GridPos { x: 1, y: 1 } })
            );
        let mut channel_1 = Channel::<Item>::default()
            .add_port(
                PortType::Input, 
                Port::<Item>::default()
                    .configure_target(GridSlice::Specific { pos: GridPos { x: 1, y: 1 } })
            ).add_port(
                PortType::Output, 
                Port::<Item>::default()
                    .configure_target(GridSlice::Specific { pos: GridPos { x: 1, y: 1 } })
            );

        let mut initial_slot = MaterialSlot::<Item>::default();
        initial_slot.set(Some(Item::Clay), 1);

        {
            let slot_0 = inventory_0.get_mut(SlotID(0));
            assert!(slot_0.is_some());
            let slot_0 = slot_0.unwrap();

            slot_0.insert(&mut initial_slot, 0);

            assert_eq!(slot_0.get().0, Some(Item::Clay));
        }

        let mut orders = vec![];

        channel_0.pull_order(GridPos { x: 0, y: 0 }, &mut orders);
        assert_eq!(orders.len(), 1);

        channel_0.write_order(&inventory_0, &mut orders[0]);
        assert!(orders[0].slot.is_some());

        channel_1.response_order(&mut inventory_1, &mut orders[0]);

        {
            let slot = inventory_1.get(SlotID(0));
            assert!(slot.is_some());

            let slot = slot.unwrap();
            assert_eq!(slot.get().0, Some(Item::Clay));
        }

        channel_0.check_order(&mut inventory_0, &orders[0]);

        {
            let slot = inventory_0.get(SlotID(0));
            assert!(slot.is_some());

            let slot = slot.unwrap();
            assert_eq!(slot.get().0, None);
        }
    }
    #[test]
    fn logistics_overflow_all() {
        let mut inventory_0 = Inventory::<Item>::new(1);
        let mut inventory_1 = Inventory::<Item>::new(1);

        let mut channel_0 = Channel::<Item>::default()
            .add_port(
                PortType::Input, 
                Port::<Item>::default()
                    .configure_target(GridSlice::Specific { pos: GridPos { x: 1, y: 1 } })
            ).add_port(
                PortType::Output, 
                Port::<Item>::default()
                    .configure_target(GridSlice::Specific { pos: GridPos { x: 1, y: 1 } })
            );
        let mut channel_1 = Channel::<Item>::default()
            .add_port(
                PortType::Input, 
                Port::<Item>::default()
                    .configure_target(GridSlice::Specific { pos: GridPos { x: 1, y: 1 } })
            ).add_port(
                PortType::Output, 
                Port::<Item>::default()
                    .configure_target(GridSlice::Specific { pos: GridPos { x: 1, y: 1 } })
            );

        let mut initial_slot = MaterialSlot::<Item>::default();
        initial_slot.set(Some(Item::Clay), 5000);

        {
            let slot_0 = inventory_0.get_mut(SlotID(0));
            assert!(slot_0.is_some());
            let slot_0 = slot_0.unwrap();

            slot_0.insert(&mut initial_slot, 0);

            assert_eq!(slot_0.get().0, Some(Item::Clay));
        }
        
        initial_slot.set(Some(Item::Clay), 5000);

        {
            let slot_1 = inventory_1.get_mut(SlotID(0));
            assert!(slot_1.is_some());
            let slot_1 = slot_1.unwrap();

            slot_1.insert(&mut initial_slot, 0);

            assert_eq!(slot_1.get().0, Some(Item::Clay));
        }

        let mut orders = vec![];

        channel_0.pull_order(GridPos { x: 0, y: 0 }, &mut orders);
        assert_eq!(orders.len(), 1);

        channel_0.write_order(&inventory_0, &mut orders[0]);
        assert!(orders[0].slot.is_some());

        channel_1.response_order(&mut inventory_1, &mut orders[0]);

        {
            let slot = inventory_1.get(SlotID(0));
            assert!(slot.is_some());

            let slot = slot.unwrap();
            assert_eq!(slot.get().0, Some(Item::Clay));
            assert_eq!(slot.get().1, 9999);
        }

        channel_0.check_order(&mut inventory_0, &orders[0]);

        {
            let slot = inventory_0.get(SlotID(0));
            assert!(slot.is_some());

            let slot = slot.unwrap();
            assert_eq!(slot.get().0, Some(Item::Clay));
            assert_eq!(slot.get().1, 1);
        }
    }
}
