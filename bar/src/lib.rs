extern crate foo;

use foo::*;

pub struct Bar {}

// FIXME:
// This fails as both OtherTypes are considered the same type
// by the compiler, but actually are not. It only fails if Foo
// is in another crate!
impl Something<<Foo as Mapper>::OtherType> for Bar {}
impl Something<<Bar as Mapper>::OtherType> for Bar {}

// The following works and is equivalent
// impl Something<Foo> for Bar {}
// impl Something<Bar> for Bar {}

impl Mapper for Bar {
    type OtherType = Bar;
}

pub struct Baz { }

impl Mapper for Baz {
    type OtherType = Baz;
}

// FIXME:
// This fails the same, but only for Foo that is in another crate
// commenting out the first line makes it work
impl Something<<Foo as Mapper>::OtherType> for Baz {}
impl Something<<Bar as Mapper>::OtherType> for Baz {}
impl Something<<Baz as Mapper>::OtherType> for Baz {}

// This works
// impl Something<Foo> for Baz {}
// impl Something<Bar> for Baz {}
// impl Something<Baz> for Baz {}
