// Copyright 2013 The Lmath Developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[link(name = "lmath",
       vers = "0.1.0",
       uuid = "A2DEEB53-EB35-4B44-B247-9044B57E3BA5",
       author = "Brendan Zabarauskas",
       url = "https://github.com/bjz/lmath-rs")];

#[comment = "A generic linear algebra library."];
#[license = "ASL2"];
#[crate_type = "lib"];

pub mod mat;
pub mod quat;
pub mod vec;

pub mod projection;

pub trait Dimensional<T,Slice> {
    pub fn index<'a>(&'a self, i: uint) -> &'a T;
    pub fn index_mut<'a>(&'a mut self, i: uint) -> &'a mut T;
    pub fn as_slice<'a>(&'a self) -> &'a Slice;
    pub fn as_mut_slice<'a>(&'a mut self) -> &'a mut Slice;
    pub fn map(&self, f: &fn(&T) -> T) -> Self;
    pub fn map_mut(&mut self, f: &fn(&mut T));
}