//! Shaku should support any number of submodules. Because the module macro uses a tuple for the
//! submodule type, we need to make sure it works for > 12 submodules (when tuples stop having
//! common trait impls).

#![allow(clippy::too_many_arguments)]

use shaku::{module, ModuleInterface};

trait Submodule1: ModuleInterface {}
trait Submodule2: ModuleInterface {}
trait Submodule3: ModuleInterface {}
trait Submodule4: ModuleInterface {}
trait Submodule5: ModuleInterface {}
trait Submodule6: ModuleInterface {}
trait Submodule7: ModuleInterface {}
trait Submodule8: ModuleInterface {}
trait Submodule9: ModuleInterface {}
trait Submodule10: ModuleInterface {}
trait Submodule11: ModuleInterface {}
trait Submodule12: ModuleInterface {}
trait Submodule13: ModuleInterface {}
trait Submodule14: ModuleInterface {}
trait Submodule15: ModuleInterface {}

module! {
    TestModule {
        components = [],
        providers = [],

        use dyn Submodule1 { components = [], providers = [] },
        use dyn Submodule2 { components = [], providers = [] },
        use dyn Submodule3 { components = [], providers = [] },
        use dyn Submodule4 { components = [], providers = [] },
        use dyn Submodule5 { components = [], providers = [] },
        use dyn Submodule6 { components = [], providers = [] },
        use dyn Submodule7 { components = [], providers = [] },
        use dyn Submodule8 { components = [], providers = [] },
        use dyn Submodule9 { components = [], providers = [] },
        use dyn Submodule10 { components = [], providers = [] },
        use dyn Submodule11 { components = [], providers = [] },
        use dyn Submodule12 { components = [], providers = [] },
        use dyn Submodule13 { components = [], providers = [] },
        use dyn Submodule14 { components = [], providers = [] },
        use dyn Submodule15 { components = [], providers = [] },
    }
}

#[test]
fn compile_ok() {
    // Make the unused warnings on types go away
    let _: Option<TestModule> = None;
}
