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

trait Shaver {
    fn shave(&self, shavee: &mut impl BeShaved);
}

struct ShaverMan {
    beard: Beard,
}

// The type system disallows us to shave self as we shave others
impl ShaverMan {
    pub fn new() -> Self {
        Self { beard: Beard(128) }
    }
}

impl SelfShave for ShaverMan {
    fn shave_self(&mut self) {
        self.beard.0 = 0;
    }
}

impl BeShaved for ShaverMan {
    fn be_shaved(&mut self, shaver: impl FnOnce(&mut Beard)) {
        shaver(&mut self.beard);
    }
}

impl Shaver for ShaverMan {
    fn shave(&self, shavee: &mut impl BeShaved) {
        shavee.be_shaved(|beard| beard.0 = 0);
    }
}

/// has no beard;
/// should be !Shave
struct ShaverWoman;

impl Shaver for ShaverWoman {
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
    use crate::{Citizen, SelfShave, Shaver, ShaverMan};

    #[test]
    fn test_shaver_man() {
        let mut shaver = ShaverMan::new();
        let mut shavee = Citizen::new();

        shaver.shave(&mut shavee);
        shaver.shave_self();
        // This should be noted:
        //
        // ```rust
        // shaver.shave(&mut shaver);
        // ```
        //
        // is rejected by the BorrowChecker
        // since shaver is borrowed immutably as Shaver and mutably as Shave.
        // So, shaver can't modify itself the same way he would for any other.
        // This makes Barber's Paradox unrepresentable in this style.
        //
        // It is said though that Rust's type system is Turing complete.
        // That means with enough effort everything is possible.
    }
}
