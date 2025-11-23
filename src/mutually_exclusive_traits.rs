mod sealed {
    pub trait Seal {}
}

pub struct Tag1;
pub struct Tag2;

impl sealed::Seal for Tag1 {}
impl sealed::Seal for Tag2 {}

pub trait Citizen {
    type Tag: sealed::Seal;
}

pub trait ShavesHimself: Citizen<Tag = Tag1> {}
impl<T: Citizen<Tag = Tag1>> ShavesHimself for T {}

pub trait ShavesWithBarber: Citizen<Tag = Tag2> {}
impl<T: Citizen<Tag = Tag2>> ShavesWithBarber for T {}

// Now the traits Contr1 and Contr2 can't be upheld both
//
// *and*
//
// one must be upheld by any implementor of ContrBase

struct Barber;

// Now if Barber implements Citizen, which Tag should I choose?
//
// It is either Tag1 or Tag2.
// Tag1 means that Barber ShavesHimself
// Tag2 means that Barber ShavesWithBarber
//
// This is not a compile-time error since it's on the user to choose if Barber is a Citizen at all, and which type of Citizen he is.
// But, generally, it can only be solved by saying Barber is not a Citizen.

struct AssertShaves<T: ShavesHimself>(T);
struct AssertBarbes<T: ShavesWithBarber>(T);

impl Citizen for Barber {
    type Tag = Tag1;
}

// One of those must fail.
// const ASSERT_SHAVES: AssertShaves<Barber> = AssertShaves(Barber);
// const ASSERT_BARBES: AssertBarbes<Barber> = AssertBarbes(Barber);
