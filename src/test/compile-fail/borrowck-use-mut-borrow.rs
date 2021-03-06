// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(box_syntax)]

#[derive(Copy)]
struct A { a: isize, b: isize }

struct B { a: isize, b: Box<isize> }

fn var_copy_after_var_borrow() {
    let mut x: isize = 1;
    let p = &mut x;
    drop(x); //~ ERROR cannot use `x` because it was mutably borrowed
    *p = 2;
}

fn var_copy_after_field_borrow() {
    let mut x = A { a: 1, b: 2 };
    let p = &mut x.a;
    drop(x); //~ ERROR cannot use `x` because it was mutably borrowed
    *p = 3;
}

fn field_copy_after_var_borrow() {
    let mut x = A { a: 1, b: 2 };
    let p = &mut x;
    drop(x.a); //~ ERROR cannot use `x.a` because it was mutably borrowed
    p.a = 3;
}

fn field_copy_after_field_borrow() {
    let mut x = A { a: 1, b: 2 };
    let p = &mut x.a;
    drop(x.a); //~ ERROR cannot use `x.a` because it was mutably borrowed
    *p = 3;
}

fn fu_field_copy_after_var_borrow() {
    let mut x = A { a: 1, b: 2 };
    let p = &mut x;
    let y = A { b: 3, .. x }; //~ ERROR cannot use `x.a` because it was mutably borrowed
    drop(y);
    p.a = 4;
}

fn fu_field_copy_after_field_borrow() {
    let mut x = A { a: 1, b: 2 };
    let p = &mut x.a;
    let y = A { b: 3, .. x }; //~ ERROR cannot use `x.a` because it was mutably borrowed
    drop(y);
    *p = 4;
}

fn var_deref_after_var_borrow() {
    let mut x: Box<isize> = box 1;
    let p = &mut x;
    drop(*x); //~ ERROR cannot use `*x` because it was mutably borrowed
    **p = 2;
}

fn field_deref_after_var_borrow() {
    let mut x = B { a: 1, b: box 2 };
    let p = &mut x;
    drop(*x.b); //~ ERROR cannot use `*x.b` because it was mutably borrowed
    p.a = 3;
}

fn field_deref_after_field_borrow() {
    let mut x = B { a: 1, b: box 2 };
    let p = &mut x.b;
    drop(*x.b); //~ ERROR cannot use `*x.b` because it was mutably borrowed
    **p = 3;
}

fn main() {
    var_copy_after_var_borrow();
    var_copy_after_field_borrow();

    field_copy_after_var_borrow();
    field_copy_after_field_borrow();

    fu_field_copy_after_var_borrow();
    fu_field_copy_after_field_borrow();

    var_deref_after_var_borrow();
    field_deref_after_var_borrow();
    field_deref_after_field_borrow();
}
