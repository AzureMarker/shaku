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
        interfaces = [],

        use Submodule1 { components = [], providers = [], interfaces = [] },
        use Submodule2 { components = [], providers = [], interfaces = [] },
        use Submodule3 { components = [], providers = [], interfaces = [] },
        use Submodule4 { components = [], providers = [], interfaces = [] },
        use Submodule5 { components = [], providers = [], interfaces = [] },
        use Submodule6 { components = [], providers = [], interfaces = [] },
        use Submodule7 { components = [], providers = [], interfaces = [] },
        use Submodule8 { components = [], providers = [], interfaces = [] },
        use Submodule9 { components = [], providers = [], interfaces = [] },
        use Submodule10 { components = [], providers = [], interfaces = [] },
        use Submodule11 { components = [], providers = [], interfaces = [] },
        use Submodule12 { components = [], providers = [], interfaces = [] },
        use Submodule13 { components = [], providers = [], interfaces = [] },
        use Submodule14 { components = [], providers = [], interfaces = [] },
        use Submodule15 { components = [], providers = [], interfaces = [] },
    }
}

#[test]
fn compile_ok() {}
