use std::cell::RefCell;
use std::rc::Rc;
use crate::connector::Connector;
use crate::constraint::Constraint;

pub struct Probe {
    connector: Rc<RefCell<Connector>>,
}

impl Probe {
    pub fn new(connector: &Rc<RefCell<Connector>>) -> Rc<RefCell<Self>> {
        let myself =
            Rc::new(RefCell::new(Self { connector: connector.clone() }));
        connector.clone().borrow_mut().register(myself.clone());
        myself
    }
}

impl Constraint for Probe {
    fn updated(&self, _is_none: bool) {
        let connector = self.connector.borrow();
        println!("Probe of {} is now {:?}", connector.name, connector.value);
    }
}
