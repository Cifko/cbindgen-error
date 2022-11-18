#[cfg(feature = "some_feature")]
pub struct TestStruct<P>(pub P);

#[cfg(not(feature = "some_feature"))]
pub struct TestStruct<P>(pub P);

pub type Test1 = TestStruct<u8>;
pub type Test2 = TestStruct<u16>;
