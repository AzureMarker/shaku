#![allow(clippy::blacklisted_name, clippy::mutex_atomic)]
#![cfg(feature = "thread_safe")]

use rand::Rng;
use shaku::{module, Component, HasComponent, Interface};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

trait Foo: Interface {
    fn get_value(&self) -> usize;
    fn set_value(&self, _: usize);
}

#[derive(Component)]
#[shaku(interface = Foo)]
struct FooImpl {
    #[shaku(default = AtomicUsize::new(FOO_DEFAULT_VALUE))]
    value: AtomicUsize,
}

impl Foo for FooImpl {
    fn get_value(&self) -> usize {
        self.value.load(Ordering::SeqCst)
    }

    fn set_value(&self, val: usize) {
        self.value.store(val, Ordering::SeqCst)
    }
}

module! {
    FooModule {
        components = [FooImpl as dyn Foo],
        providers = []
    }
}

const FOO_DEFAULT_VALUE: usize = 17;
const NB_THREADS: usize = 10;
const MAX_SLEEP_TIME: u64 = 2000;

/// Call resolve_ref from multiple threads and verify the value is correct
#[test]
fn simple_multithreaded_resolve_ref() {
    // Build module
    let module = FooModule::builder().build();
    let shared_module = Arc::new(module);

    // Launch a few threads where each will try to resolve `Foo`
    let mut handles = Vec::new();
    for i in 0..NB_THREADS {
        let shared_module = Arc::clone(&shared_module);

        handles.push(
            thread::Builder::new()
                .name(format!("reader #{}", &i))
                .spawn(move || {
                    // Inject some randomness in the test
                    let sleep_ms = rand::thread_rng().gen_range(0..MAX_SLEEP_TIME);
                    thread::sleep(Duration::from_millis(sleep_ms));

                    let foo: &dyn Foo = shared_module.resolve_ref();
                    assert_eq!(foo.get_value(), FOO_DEFAULT_VALUE);
                })
                .unwrap(),
        );
    }

    // Wait until all the threads are done
    for handle in handles {
        handle.join().unwrap();
    }
}

/// Read and write the value from multiple threads, verifying the value on each read
#[test]
fn simple_multithreaded_resolve_ref_n_mut() {
    // Build module
    let module = FooModule::builder().build();
    let shared_module = Arc::new(module);
    let latest_data: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(FOO_DEFAULT_VALUE));

    // Launch a few threads where each will try to resolve `Foo`
    let mut handles = Vec::new();
    for i in 0..NB_THREADS {
        let (shared_module, latest_data) = (shared_module.clone(), latest_data.clone());

        handles.push(
            thread::Builder::new()
                .name(format!("reader #{}", &i))
                .spawn(move || {
                    // Inject some randomness in the test
                    let handle = thread::current();
                    let sleep_ms = rand::thread_rng().gen_range(0..MAX_SLEEP_TIME);
                    thread::sleep(Duration::from_millis(sleep_ms));

                    // Resolve the module
                    let use_mut = rand::thread_rng().gen_bool(0.5);
                    if use_mut {
                        // Set a new value
                        let foo: &dyn Foo = shared_module.resolve_ref();
                        let new_value: usize = rand::thread_rng().gen_range(0..256);
                        foo.set_value(new_value);
                        assert_eq!(foo.get_value(), new_value);

                        latest_data.store(new_value, Ordering::SeqCst);

                        println!(
                            "In thread {:?} > resolve ok > value changed to {}",
                            &handle.name().unwrap(),
                            new_value
                        );
                    } else {
                        // Read the data and check against the expected value
                        let foo: &dyn Foo = shared_module.resolve_ref();
                        let data = latest_data.load(Ordering::SeqCst);

                        println!(
                            "In thread {:?} > resolve ok > value should be {}",
                            &handle.name().unwrap(),
                            data
                        );
                        assert_eq!(foo.get_value(), data);
                    }
                })
                .unwrap(),
        );
    }

    // Wait until all the threads are done
    for handle in handles {
        handle.join().unwrap();
    }
}
