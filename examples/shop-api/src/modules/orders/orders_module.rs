use rustforge::prelude::*;

use super::{OrderController, OrderService};

#[module(controllers = [OrderController], providers = [OrderService])]
pub struct OrdersModule;
