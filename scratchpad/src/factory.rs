// This is for you to implement your factory.
// Later, you should use this inside of this crate's main.rs file. (it should already be done for you)
// Hint: be sure to declare types as public! (`pub`)

use std::collections::HashMap;
use std::fmt::Debug;

trait Process {
    // Note the use of associated functions as a means to "configure" our belt
    type RawMaterial;
    type Product: Debug + Clone;
    /// Processes the item from a raw material to a product.
    fn push_along_the_belt(&self, material: Self::RawMaterial) -> Self::Product;

    // The raw material to process on this belt
    fn material(&self) -> Self::RawMaterial;
}

/// What a factory looks like.  Each factory can only process one type of thing, in theory we could
// "parallelize" these. You can think of these belts as parachains on Polkadot, which execute transactions horizontally.
// Note that we wanted a multi-process factory, we would need to use dynamic dispatch for both processes (Vec<Box<dyn Process>>)
pub struct Factory<P: Process> {
    factory_id: u32,
    processes: Vec<P>,
    completed_products: HashMap<usize, P::Product>,
}

impl<P: Process> Factory<P> {
    // Our "constructor"
    pub fn new(factory_id: u32) -> Self {
        Factory { factory_id, processes: vec![], completed_products: HashMap::new() }
    }

    pub fn process_all(&mut self) {
        // loop thru and run "push_along_belt", then push the products to completed products.
        for (id, process) in self.processes.iter().enumerate() {
            println!("Processing {}...", id);
            let product: <P as Process>::Product = process.push_along_the_belt(process.material());
            self.completed_products.insert(id, product.clone());
            println!("Process successful, product produced! {:?}...", product);
        }
    }

    pub fn add_new_process(&mut self, process: P) {
        self.processes.push(process)
    }

    // Run the factory
    pub fn run(&mut self) {
        // run all belts
        self.process_all();
    }
}

// Look at all this yucky boilerplate...
// I wonder if there is a better way to do this (there is - macros!)

// Also, the amount here is in tons
pub struct IronOre {
    pub amount: u32,
}

#[derive(Debug, Clone)]
pub struct Steel {
    pub amount: u32,
}

pub struct OreProcess {
    pub amount: u32,
}

impl Process for OreProcess {
     // Note the use of associated functions as a means to "configure" our belt
     type RawMaterial = IronOre;
     type Product = Steel;
     
     /// Processes the item from a raw material to a product.
     fn push_along_the_belt(&self, material: Self::RawMaterial) -> Self::Product {
        Steel {amount: material.amount /2 }
     }

       // The raw material to process on this belt
    fn material(&self) -> Self::RawMaterial {
        IronOre {amount: self.amount}

    }
}

// TODO: Implement Process for OreProcess (hint.. factory/src/main.rs)
