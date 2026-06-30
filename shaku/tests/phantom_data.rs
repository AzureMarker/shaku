use shaku::{module, Component, HasComponent, HasProvider, Interface, Provider};
use std::error::Error as StdError;
use std::fmt;
use std::marker::PhantomData;
use std::ops::Deref;
use std::rc::Rc;

trait ComponentTrait: Interface {
    fn value(&self) -> usize;
}

#[derive(Component)]
#[shaku(interface = ComponentTrait)]
struct PhantomComponent<T: 'static> {
    #[shaku(default = 17)]
    value: usize,
    #[shaku(phantom)]
    marker: PhantomData<fn() -> T>,
}

impl<T: 'static> ComponentTrait for PhantomComponent<T> {
    fn value(&self) -> usize {
        self.value
    }
}

#[derive(Debug)]
struct CommandError;

impl fmt::Display for CommandError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "command failed")
    }
}

impl StdError for CommandError {}

#[derive(Debug)]
struct SmartCardError;

impl fmt::Display for SmartCardError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "smartcard failed")
    }
}

impl StdError for SmartCardError {}

impl From<SmartCardError> for CommandError {
    fn from(_: SmartCardError) -> Self {
        Self
    }
}

trait SmartCard: Send + Sync {
    fn run(&self, input: &[u8]) -> Result<Vec<u8>, SmartCardError>;
}

#[derive(Provider)]
#[shaku(interface = SmartCard)]
struct SmartCardImpl;

impl SmartCard for SmartCardImpl {
    fn run(&self, input: &[u8]) -> Result<Vec<u8>, SmartCardError> {
        let mut output = b"echo: ".to_vec();
        output.extend_from_slice(input);
        Ok(output)
    }
}

trait RunCommand<Input, Output>: Interface
where
    Input: Deref<Target = [u8]>,
    Output: for<'a> From<&'a [u8]>,
{
    fn run(&self, input: &Input) -> Result<Output, CommandError>;
}

#[derive(Provider)]
#[shaku(interface = RunCommand<Input, Output>)]
struct RunCommandExecuter<Input, Output>
where
    Input: Deref<Target = [u8]> + 'static,
    Output: for<'a> From<&'a [u8]> + 'static,
{
    #[shaku(phantom)]
    _input: PhantomData<fn() -> Input>,
    #[shaku(phantom)]
    _output: PhantomData<fn() -> Output>,
    #[shaku(provide)]
    smartcard: Box<dyn SmartCard>,
}

impl<Input, Output> RunCommand<Input, Output> for RunCommandExecuter<Input, Output>
where
    Input: Deref<Target = [u8]> + 'static,
    Output: for<'a> From<&'a [u8]> + 'static,
{
    fn run(&self, input: &Input) -> Result<Output, CommandError> {
        let input: &[u8] = input;
        let output = self.smartcard.run(input).map_err(CommandError::from)?;

        Ok(Output::from(output.as_slice()))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct CommandOutput(Rc<[u8]>);

impl From<&[u8]> for CommandOutput {
    fn from(value: &[u8]) -> Self {
        Self(Rc::from(value))
    }
}

module! {
    TestModule {
        components = [PhantomComponent<Rc<()>>],
        providers = [SmartCardImpl, RunCommandExecuter<Rc<[u8]>, CommandOutput>]
    }
}

#[test]
fn component_phantom_data_is_initialized() {
    let module = TestModule::builder().build();
    let component: &dyn ComponentTrait = module.resolve_ref();

    assert_eq!(component.value(), 17);
}

#[test]
fn provider_phantom_data_is_initialized() {
    let module = TestModule::builder().build();
    let provider: Box<dyn RunCommand<Rc<[u8]>, CommandOutput>> = module.provide().unwrap();
    let input = Rc::from(&b"ping"[..]);

    let output = provider.run(&input).unwrap();

    assert_eq!(output, CommandOutput(Rc::from(&b"echo: ping"[..])));
}
