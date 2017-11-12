pub trait Something<T> { }

pub struct Foo {}

pub trait Mapper {
    type OtherType;
}

impl Something<Foo> for Foo {}

impl Mapper for Foo {
    type OtherType = Foo;
}
