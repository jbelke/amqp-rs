// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ops::Deref;
use inflections::Inflect;
use specs::{ClassField, ClassMethodField};

use common::domain::Domain;

#[derive(Debug, Clone)]
pub struct Field<T>
    where T: BasicField
{
    field: T,
    var_name: String,
    ty: Domain,
}

impl<T> Field<T>
    where T: BasicField
{
    pub fn from_amqp0_field(
        field: T,
        ty: Domain
    ) -> Self {
        let var_name = match field.name() {
            "type" => "ty".into(),
            "nowait" => "no_wait".into(),
            name => name.to_snake_case().into(),
        };

        Field {
            field: field,
            var_name: var_name,
            ty: ty,
        }
    }

    pub fn var_name(&self) -> &str {
        &self.var_name
    }

    pub fn ty(&self) -> &Domain {
        &self.ty
    }
}

impl<T> Deref for Field<T>
    where T: BasicField
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.field
    }
}

pub trait BasicField {
    fn name(&self) -> &'static str;
}

impl BasicField for ClassMethodField {
    fn name(&self) -> &'static str {
        self.name()
    }
}

impl BasicField for ClassField {
    fn name(&self) -> &'static str {
        self.name()
    }
}