use std::cell::RefCell;
use std::rc::Rc;
use crate::connector::Connector;

pub trait Constraint {
    fn updated(&self, is_none: bool);
}

impl dyn Constraint {
    pub fn clear(connectors: Vec<&Rc<RefCell<Connector>>>) {
        connectors.iter().for_each(|c| {
            Connector::set_value(&c, None);
        });
    }

    pub fn register(connectors: Vec<&Rc<RefCell<Connector>>>,
                    constraint: &Rc<RefCell<dyn Constraint>>) {
        connectors.iter().for_each(|c| {
            c.clone().borrow_mut().register(constraint.clone());
        });
    }
}