use enum_plus_derive::EnumPlus;


#[derive(EnumPlus, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum SignalError {
    MemoryOutOfBounds = 1,

    ChildUnexist = 2,

    SignalNotFound = 3,

    ThreadNotFound = 4,

    StackEmpty = 5,

    ThreadNotInterrupted = 6,
}
