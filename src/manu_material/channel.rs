use crate::prelude::*;

#[derive(Component)]
pub struct Channel<T>
where 
    T: ManuMaterial
{
    input : Vec<Port<T>>,
    output: Vec<Port<T>>,
    open  : Vec<Port<T>>,
    pull  : Vec<Port<T>>,
}

#[derive(Component)]
pub struct Port<T> 
where 
    T: ManuMaterial
{
    filter: MaterialFilter<T>,
    slot  : InventorySlice,
    mode  : PortMode,
    target: GridSlice,
    used  : bool,
}

#[derive(Component, Default)]
pub enum PortMode {
    #[default]
    Anytime,
    WithCoolDown {
        tick: u64,
        process: u64,
    },
}

impl<T> Channel<T>
where 
    T: ManuMaterial
{
    pub fn add_port(mut self, port_type: PortType, port: Port<T>) -> Self {
        match port_type {
            PortType::Input => &mut self.input,
            PortType::Output => &mut self.output,
            PortType::Open => &mut self.open,
            PortType::Pull => &mut self.pull,
        }.push(port);
        
        self
    }
    pub fn get_ports(&self, port_type: PortType) -> &Vec<Port<T>> {
        match port_type {
            PortType::Input => &self.input,
            PortType::Output => &self.output,
            PortType::Open => &self.open,
            PortType::Pull => &self.pull,
        }
    }
    pub fn pull_order(&mut self, pos: GridPos, orders: &mut Vec<LogisticsOrder<T>>) {
        for (id, port) in self.output.iter_mut().enumerate() {
            port.reg_output_order(pos, orders, id);
        }
        for (id, port) in self.pull.iter_mut().enumerate() {
            port.reg_pull_order(pos, orders, id);
        }
    }
    pub fn write_order(&mut self, inventory: &Inventory<T>, order: &mut LogisticsOrder<T>) {
        match order.logistics_type {
            LogisticsType::InputOutput => {
                if let Some(port) = self.output.get_mut(order.client_id) {
                    order.write(port.get_first_buff(inventory));
                } else {
                    panic!("Output による LogisticsOrder 作成のロジックに不備があります（不明な index）");
                }
            },
            LogisticsType::OpenPull => {
                if let Some(port) = self.open.get_mut(order.client_id) {
                    order.write(port.get_first_buff(inventory));
                } else {
                    panic!("Pull による LogisticsOrder 作成のロジックに不備があります（不明な index）");
                }
            },
        }
    }
    pub fn response_order(&mut self, inventory: &mut Inventory<T>, order: &mut LogisticsOrder<T>) {
        let Some(slot) = &mut order.slot else {return;};
        
        match order.logistics_type {
            LogisticsType::InputOutput => {
                for port in self.input.iter_mut() {
                    port.insert(inventory, &mut slot.slot);
                }
            },
            LogisticsType::OpenPull => {
                for port in self.pull.iter_mut() {
                    if port.insert(inventory, &mut slot.slot) {
                        port.used();
                        break;
                    }
                }
            },
        }
    }
    pub fn check_order(&mut self, inventory: &mut Inventory<T>, order: &LogisticsOrder<T>) {
        if let Some(buff) = &order.slot {
            inventory.apply_buff(buff);
        }

        if order.is_done()
        && order.logistics_type == LogisticsType::OpenPull 
        && let Some(port) = self.pull.get_mut(order.client_id) {
            port.used();
        }
    }
}

impl<T> Port<T>
where 
    T: ManuMaterial
{
    pub fn reg_output_order(&mut self, from: GridPos, orders: &mut Vec<LogisticsOrder<T>>, client_id: usize) {
        self.used = false;
        for to in self.target.get_vec(from) {
            orders.push(LogisticsOrder::new(from, to, LogisticsType::InputOutput, client_id));
        }
    }
    pub fn reg_pull_order(&mut self, to: GridPos, orders: &mut Vec<LogisticsOrder<T>>, client_id: usize) {
        self.used = false;
        for from in self.target.get_vec(to) {
            orders.push(LogisticsOrder::new(from, to, LogisticsType::OpenPull, client_id));
        }
    }
    pub fn get_first_buff(&mut self, inventory: &Inventory<T>) -> Option<MaterialSlotBuff<T>> {
        if self.used {return None;}
        for id in self.slot.get_slot_id(inventory) {
            if let Some(slot) = inventory.get(*id)
            && let Some(value) = slot.get().0 
            && self.filter.check(value) {
                return Some(MaterialSlotBuff::<T>::new(slot.clone(), *id));
            }
        }
        None
    }
    pub fn insert(&mut self, inventory: &mut Inventory<T>, slot: &mut MaterialSlot<T>) -> bool {
        let mut result = false;
        for id in self.slot.get_slot_id(inventory).iter() {
            if let Some(to_slot) = inventory.get_mut(*id) {
                result |= to_slot.insert(slot);
            }
        }
        result
    }
    pub fn used(&mut self) {
        self.used = true;
    }
    pub fn configure_filter(mut self, filter: MaterialFilter<T>) -> Self {
        self.filter = filter;
        self
    }
    pub fn configure_mode(mut self, mode: PortMode) -> Self {
        self.mode = mode;
        self
    }
    pub fn configure_target(mut self, target: GridSlice) -> Self {
        self.target = target;
        self
    }
    pub fn configure_slot(mut self, slot: InventorySlice) -> Self {
        self.slot = slot;
        self
    }
}
impl<T> Default for Port<T> 
where 
    T: ManuMaterial
{
    fn default() -> Self {
        Self {
            filter: Default::default(),
            mode: Default::default(),
            target: Default::default(),
            slot: Default::default(),
            used: false,
        }
    }
}

impl<T> Default for Channel<T>
where 
    T: ManuMaterial
{
    fn default() -> Self {
        Self {
            output: Default::default(),
            input: Default::default(),
            open: Default::default(),
            pull: Default::default(),
        }
    }
}
