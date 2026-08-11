#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test] 
    fn normal_insert() {
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
    fn overflow_insert() {
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
    fn unable_insert() {
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
}
