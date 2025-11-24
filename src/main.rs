mod comptime;
mod mutually_exclusive_traits;

// Real-world design

struct Beard(u8);

trait SelfShave {
    fn shave_self(&mut self);
}

trait BeShaved {
    fn be_shaved(&mut self, fun: impl FnOnce(&mut Beard));
}

trait Barber {
    fn shave(&self, shavee: &mut impl BeShaved);
}

struct BarberMan {
    beard: Beard,
}

// The type system disallows us to shave self as we shave others
impl BarberMan {
    pub fn new() -> Self {
        Self { beard: Beard(128) }
    }
}

impl SelfShave for BarberMan {
    fn shave_self(&mut self) {
        self.beard.0 = 0;
    }
}

impl BeShaved for BarberMan {
    fn be_shaved(&mut self, barber: impl FnOnce(&mut Beard)) {
        barber(&mut self.beard);
    }
}

impl Barber for BarberMan {
    fn shave(&self, shavee: &mut impl BeShaved) {
        shavee.be_shaved(|beard| beard.0 = 0);
    }
}

/// has no beard;
/// should be !Shave
struct BarberWoman;

impl Barber for BarberWoman {
    fn shave(&self, shavee: &mut impl BeShaved) {
        shavee.be_shaved(|beard| beard.0 = 5);
    }
}

struct Citizen {
    beard: Option<Beard>,
}

impl Citizen {
    fn new() -> Self {
        Self {
            beard: Some(Beard(64)),
        }
    }
}

impl BeShaved for Citizen {
    fn be_shaved(&mut self, fun: impl FnOnce(&mut Beard)) {
        self.beard.as_mut().map(fun);
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::{Barber, BarberMan, Citizen, SelfShave};

    #[test]
    fn test_barber_man() {
        let mut barber = BarberMan::new();
        let mut shavee = Citizen::new();

        barber.shave(&mut shavee);
        barber.shave_self();
        // This should be noted:
        //
        // ```rust
        // Barber.shave(&mut Barber);
        // ```
        //
        // is rejected by the BorrowChecker
        // since Barber is borrowed immutably as Barber and mutably as Shave.
        // So, Barber can't modify itself the same way he would for any other.
        // This makes Barber's Paradox unrepresentable in this style.
        //
        // It is said though that Rust's type system is Turing complete.
        // That means with enough effort everything is possible.
    }
}
