#![allow(dead_code, unused)]

pub struct ShavesSelf<T>(pub T)
where
    Barber: Shaves<T>,
    <Barber as Shaves<T>>::Shave: HasNoShave;

pub struct NeverShavesSelf<T>(pub T)
where
    Barber: Shaves<T>,
    <Barber as Shaves<T>>::Shave: HasShave;

pub struct Barber;

pub trait HasShave {}
pub trait HasNoShave {}

impl HasShave for CanShave {}
impl HasNoShave for CanNotShave {}

pub struct CanShave;
pub struct CanNotShave;

/// Seal trait: only CanShave and CanNotShave must implement it.
trait ShaveAbility {}

impl ShaveAbility for CanShave {}
impl ShaveAbility for CanNotShave {}

pub trait Shaves<T> {
    type Shave: ShaveAbility;
}

impl<T> Shaves<ShavesSelf<T>> for Barber
where
    Barber: Shaves<T>,
    <Barber as Shaves<T>>::Shave: HasNoShave,
{
    // T wouldn't be here if Barber CanShave T
    type Shave = <Barber as Shaves<T>>::Shave;
}

impl<T> Shaves<NeverShavesSelf<T>> for Barber
where
    Barber: Shaves<T>,
    <Barber as Shaves<T>>::Shave: HasShave,
{
    // T wouldn't be here if Barber CanNotShave T
    type Shave = <Barber as Shaves<T>>::Shave;
}

impl Shaves<Barber> for Barber {
    type Shave = CanNotShave;
}

type BarberShaves<C> = <Barber as Shaves<C>>::Shave;

// /// Barber can't shave a Barber that ShavesSelf
// ///
// /// For any T:
// ///     ShavesSelf<T> can't exist in the same program that NeverShaves<T>.
// ///
// /// Why?
// /// `ShavesSelf<T>`      => `T` has `<Barber: Shaves<T>>::Shave` equal to `CanNotShave`
// /// `NeverShavesSelf<T>` => `T` has `<Barber: Shaves<T>>::Shave` equal to `CanShave`
// /// which can't be simultaneously true.
// const SHAVES_BARBER: BarberShaves<ShavesSelf<Barber>> = CanNotShave;

// /// Barber can't shave a Barber that NeverShavesSelf
// ///
// /// For any T:
// ///     ShavesSelf<T> can't exist in the same program that NeverShaves<T>.
// ///
// /// Why?
// /// `ShavesSelf<T>`      => `T` has `<Barber: Shaves<T>>::Shave` equal to `CanNotShave`
// /// `NeverShavesSelf<T>` => `T` has `<Barber: Shaves<T>>::Shave` equal to `CanShave`
// /// which can't be simultaneously true.
// const NEVER_SHAVES_BARBER: BarberShaves<NeverShavesSelf<Barber>> = CanShave;

// Ok, if Barber CanNotShave Barber, then Barber ShavesSelf
//
// Now we know that Barber ShavesSelf, that implies that Barber CanNotShave Barber.
//
// That implies that ShavesSelf<ShavesSelf<Barber>> must not exist as Barber
const CHECK_NON_SHAVES: BarberShaves<ShavesSelf<ShavesSelf<Barber>>> = CanNotShave;
// const CHECK_AGAIN: BarberShaves<NeverShavesSelf<ShavesSelf<Barber>>> = CanNotShave;

// const CHECK_SHAVES: BarberShaves<NeverShavesSelf<Barber>> = CanShave;

// const SHAVING_BARBER: ShavesSelf<Barber> = ShavesSelf(Barber);
// const NONSHAVING_BARBER: NeverShavesSelf<Barber> = NeverShavesSelf(Barber);

#[cfg(test)]
mod tests {
    use crate::comptime::{Barber, NeverShavesSelf, ShavesSelf};

    #[test]
    fn test_shaves() {
        // Here Barber must Shave<Barber>, and that's done.
        // and since there's a blanket implementation that Barber must Shave any ShavesSelf,
        // but barber can't since some Barber already
    }
}
