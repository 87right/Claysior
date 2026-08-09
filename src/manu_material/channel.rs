use crate::prelude::*;

#[derive(Component, Default)]
pub struct Channel<T>
where 
    T: ManuMaterial
{
    input : Vec<Port<T>>,
    output: Vec<Port<T>>,
    open  : Vec<Port<T>>,
    pull  : Vec<Port<T>>,
}

#[derive(Component, Default)]
pub struct Port<T> 
where 
    T: ManuMaterial
{
    filter: MaterialFilter<T>,
    slot  : InventorySlice,
    mode  : PortMode,
    target: GridSlice,
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
    #![allow(unused)]
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
    pub fn pull_order(&self, inventory: &Inventory<T>, pos: GridPos, orders: &mut Vec<LogisticsOrder<T>>) {
        for port in &self.output {
            port.pull_output_order(inventory, pos, orders);
        }
        for port in &self.pull {
            port.pull_pull_order(inventory, pos, orders);
        }
    }
    pub fn write_order(&self, inventory: &Inventory<T>, order: &mut LogisticsOrder<T>) {

    }
    pub fn response_order(&mut self, inventory: &mut Inventory<T>, order: &mut LogisticsOrder<T>) {

    }
    pub fn check_order(&mut self, inventory: &mut Inventory<T>, order: &LogisticsOrder<T>) {

    }
}

impl<T> Port<T>
where 
    T: ManuMaterial
{
    #![allow(unused)]
    pub fn pull_output_order(&self, inventory: &Inventory<T>, pos: GridPos, orders: &mut Vec<LogisticsOrder<T>>) {

    }
    pub fn pull_pull_order(&self, inventory: &Inventory<T>, pos: GridPos, orders: &mut Vec<LogisticsOrder<T>>) {

    }
}
