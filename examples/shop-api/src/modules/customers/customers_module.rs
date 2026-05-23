use rustforge::prelude::*;

use super::{CustomerController, CustomerService};

#[module(controllers = [CustomerController], providers = [CustomerService])]
pub struct CustomersModule;
