pub struct ShavesSelf<T>(pub T)
where
    Barber: Shaves<T>,
    <Barber as Shaves<T>>::Shave: HasNoShave;

pub struct NeverShavesSelf<T>(pub T)
where
    Barber: Shaves<T>,
    <Barber as Shaves<T>>::Shave: HasShave;

pub struct Barber;

trait HasShave {}
trait HasNoShave {}

impl HasShave for CanShave {}
impl HasNoShave for CanNotShave {}

pub struct CanShave;
pub struct CanNotShave;

pub trait ShaveAbility {}

impl ShaveAbility for CanShave {}
impl ShaveAbility for CanNotShave {}

trait Shaves<T> {
    type Shave: ShaveAbility;
}

impl<T> Shaves<ShavesSelf<T>> for Barber
where
    Barber: Shaves<T>,
    <Barber as Shaves<T>>::Shave: HasNoShave,
{
    type Shave = CanNotShave;
}

impl<T> Shaves<NeverShavesSelf<T>> for Barber
where
    Barber: Shaves<T>,
    <Barber as Shaves<T>>::Shave: HasShave,
{
    type Shave = CanShave;
}

impl Shaves<Barber> for Barber {
    type Shave = CanNotShave;
}

type BarberShaves<C> = <Barber as Shaves<C>>::Shave;

/// Barber can't shave a Barber that ShavesSelf
///
/// For any T:
///     ShavesSelf<T> can't exist in the same program that NeverShaves<T>.
/// Why?
/// `ShavesSelf<T>`      => `T` has `<Barber: Shaves<T>>::Shave` equal to `CanNotShave`
/// `NeverShavesSelf<T>` => `T` has `<Barber: Shaves<T>>::Shave` equal to `CanShave`
/// which can't be simultaneously true.
const SHAVES_BARBER: BarberShaves<ShavesSelf<Barber>> = CanNotShave;

/// Barber can't shave a Barber that NeverShavesSelf
///
/// For any T:
///     ShavesSelf<T> can't exist in the same program that NeverShaves<T>.
/// Why?
/// `ShavesSelf<T>`      => `T` has `<Barber: Shaves<T>>::Shave` equal to `CanNotShave`
/// `NeverShavesSelf<T>` => `T` has `<Barber: Shaves<T>>::Shave` equal to `CanShave`
/// which can't be simultaneously true.
const NEVER_SHAVES_BARBER: BarberShaves<NeverShavesSelf<Barber>> = CanShave;
