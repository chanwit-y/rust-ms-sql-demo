//  pub mod util {

    pub struct Util {
       pub name: String,
    }

    impl Util {
        pub fn display_name(&self) {
            print!("{}", self.name);
        }
    }
// }
// 