use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use crate::constraint::Constraint;

pub struct Connector {
    pub name: String,
    pub value: Option<f64>,
    constraints: Vec<Rc<RefCell<dyn Constraint>>>,
    is_constant: bool,
}

impl Connector {
    pub fn new(name: String) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(
            Self { name, value: None, constraints: vec![], is_constant: false }))
    }

    pub fn register(&mut self, constraint: Rc<RefCell<dyn Constraint>>) {
        self.constraints.push(constraint);
    }

    pub fn update(&mut self, value: Option<f64>, setting_by_constant: bool) {
        self.value = value;
        self.is_constant = setting_by_constant;
    }

    pub fn inform(&self, is_none: bool) {
        self.constraints.iter().for_each(|c| {
            c.borrow().updated(is_none);
        })
    }

    pub fn set_value(connector: &Rc<RefCell<Connector>>, value: Option<f64>) {
        Connector::set_value_internal(connector, value, false);
    }

    pub fn fix(connector: &Rc<RefCell<Connector>>, value: f64) {
        Connector::set_value_internal(connector, Some(value), true);
    }

    fn set_value_internal(connector: &Rc<RefCell<Connector>>, value: Option<f64>, setting_by_constant: bool) {
        // println!("32. Connector {} try to set_value {:?}", connector.borrow().name, value);
        if connector.borrow().is_constant { return; }
        if let Some(v) = connector.borrow().value {
            if Some(v) == value {
                return;
            }
        }
        if let Err(_) = connector.try_borrow_mut() {
            // println!("Connector {} is already borrowed", connector.borrow().name);
            return;
        }
        // println!("32. Connector {} set_value {:?}", connector.borrow().name, value);
        connector.borrow_mut().update(value, setting_by_constant);
        connector.borrow().inform(value.is_none());
    }
}

impl Debug for Connector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Connector({}, {:?})", self.name, self.value)
    }
}

impl PartialEq<Self> for Connector {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Connector {}