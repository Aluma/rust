// Copyright 2013-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Tests that a heterogeneous list of existential types can be put inside an Arc
// and shared between tasks as long as all types fulfill Send.

// ignore-pretty

#![allow(unknown_features)]
#![feature(box_syntax, std_misc)]
#![feature(unboxed_closures)]

use std::sync::Arc;
use std::sync::mpsc::channel;
use std::thread::Thread;

trait Pet {
    fn name(&self, blk: Box<FnMut(&str)>);
    fn num_legs(&self) -> usize;
    fn of_good_pedigree(&self) -> bool;
}

struct Catte {
    num_whiskers: usize,
    name: String,
}

struct Dogge {
    bark_decibels: usize,
    tricks_known: usize,
    name: String,
}

struct Goldfyshe {
    swim_speed: usize,
    name: String,
}

impl Pet for Catte {
    fn name(&self, mut blk: Box<FnMut(&str)>) { blk(&self.name) }
    fn num_legs(&self) -> usize { 4 }
    fn of_good_pedigree(&self) -> bool { self.num_whiskers >= 4 }
}
impl Pet for Dogge {
    fn name(&self, mut blk: Box<FnMut(&str)>) { blk(&self.name) }
    fn num_legs(&self) -> usize { 4 }
    fn of_good_pedigree(&self) -> bool {
        self.bark_decibels < 70 || self.tricks_known > 20
    }
}
impl Pet for Goldfyshe {
    fn name(&self, mut blk: Box<FnMut(&str)>) { blk(&self.name) }
    fn num_legs(&self) -> usize { 0 }
    fn of_good_pedigree(&self) -> bool { self.swim_speed >= 500 }
}

pub fn main() {
    let catte = Catte { num_whiskers: 7, name: "alonzo_church".to_string() };
    let dogge1 = Dogge {
        bark_decibels: 100,
        tricks_known: 42,
        name: "alan_turing".to_string(),
    };
    let dogge2 = Dogge {
        bark_decibels: 55,
        tricks_known: 11,
        name: "albert_einstein".to_string(),
    };
    let fishe = Goldfyshe {
        swim_speed: 998,
        name: "alec_guinness".to_string(),
    };
    let arc = Arc::new(vec!(box catte  as Box<Pet+Sync+Send>,
                            box dogge1 as Box<Pet+Sync+Send>,
                            box fishe  as Box<Pet+Sync+Send>,
                            box dogge2 as Box<Pet+Sync+Send>));
    let (tx1, rx1) = channel();
    let arc1 = arc.clone();
    let _t1 = Thread::spawn(move|| { check_legs(arc1); tx1.send(()); });
    let (tx2, rx2) = channel();
    let arc2 = arc.clone();
    let _t2 = Thread::spawn(move|| { check_names(arc2); tx2.send(()); });
    let (tx3, rx3) = channel();
    let arc3 = arc.clone();
    let _t3 = Thread::spawn(move|| { check_pedigree(arc3); tx3.send(()); });
    rx1.recv();
    rx2.recv();
    rx3.recv();
}

fn check_legs(arc: Arc<Vec<Box<Pet+Sync+Send>>>) {
    let mut legs = 0;
    for pet in &*arc {
        legs += pet.num_legs();
    }
    assert!(legs == 12);
}
fn check_names(arc: Arc<Vec<Box<Pet+Sync+Send>>>) {
    for pet in &*arc {
        // FIXME (#22405): Replace `Box::new` with `box` here when/if possible.
        pet.name(Box::new(|name| {
            assert!(name.as_bytes()[0] == 'a' as u8 && name.as_bytes()[1] == 'l' as u8);
        }))
    }
}
fn check_pedigree(arc: Arc<Vec<Box<Pet+Sync+Send>>>) {
    for pet in &*arc {
        assert!(pet.of_good_pedigree());
    }
}
