use shaku::{module, Component, HasComponent, Interface};
use std::sync::Arc;

trait Presenter: Interface {
    fn register_paths(&self);
}
trait Test: Interface {
    fn test(&self) -> i32;
}
trait App: Interface {
    fn run(&self);
}
trait IRouter: Interface {
    fn register_paths(&self);
}

#[derive(Component)]
#[shaku(interface = IRouter)]
struct Router {
    #[shaku(collect)]
    presenters: Vec<Arc<dyn Presenter>>,
}
impl IRouter for Router {
    fn register_paths(&self) {
        assert_eq!(self.presenters.len(), 2);
        for p in self.presenters.iter() {
            p.register_paths();
        }
    }
}

#[derive(Component)]
#[shaku(interface = App)]
struct Builder {
    #[shaku(inject)]
    router: Arc<dyn IRouter>,
}
impl App for Builder {
    fn run(&self) {
        self.router.register_paths();
    }
}

#[derive(Component)]
#[shaku(interface = Presenter)]
struct P1 {}
impl Presenter for P1 {
    fn register_paths(&self) {}
}
#[derive(Component)]
#[shaku(interface = Presenter)]
struct P2 {}
impl Presenter for P2 {
    fn register_paths(&self) {}
}
#[derive(Component)]
#[shaku(interface = Test)]
struct T1 {}
impl Test for T1 {
    fn test(&self) -> i32 {
        return 1;
    }
}
#[derive(Component)]
#[shaku(interface = Test)]
struct T2 {}
impl Test for T2 {
    fn test(&self) -> i32 {
        return 2;
    }
}
#[derive(Component)]
#[shaku(interface = Tst)]
struct TstImpl {
    #[shaku(collect)]
    presenters: Vec<Arc<dyn Test>>,
    #[allow(dead_code)]
    #[shaku(collect)]
    p2: Vec<Arc<dyn Presenter>>,
}

trait Tst: Interface {
    fn tst(&self) -> i32;
}
impl Tst for TstImpl {
    fn tst(&self) -> i32 {
        let mut sum = 0;
        assert_eq!(self.presenters.len(), 2);

        for p in self.presenters.iter() {
            sum += p.test()
        }
        return sum;
    }
}

module! {
    TestModule {
        components = [Builder, Router],
        providers = [],
        interfaces = [#[implementations P1, P2] dyn Presenter]
    }
}

module! {
    TestModule2 {
        components = [TstImpl],
        providers = [],
        interfaces = [#[implementations P1, P2] dyn Presenter, #[implementations T1, T2] dyn Test],
    }
}

#[test]
fn interfaces() {
    let module = TestModule::builder().build();
    let app: &dyn App = module.resolve_ref();
    app.run();

    let module = TestModule2::builder().build();
    let app: &dyn Tst = module.resolve_ref();
    assert_eq!(app.tst(), 3);
}
