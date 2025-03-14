use enum_plus_derive::EnumPlus;

#[derive(EnumPlus, Debug, PartialEq, Eq)]
enum Code {
    ADD = 0x00,
    SUB = 0x01,
    AND = 0x02,
    OR = 0x03,
}

#[test]
fn test_try_from_u8() {
    assert_eq!(0u8.try_into(), Ok(Code::ADD));
    assert_eq!(1u8.try_into(), Ok(Code::SUB));
    assert_eq!(0xFFu8.try_into() as Result<Code, ()>, Err(()));
}

#[test]
fn test_try_from_str() {
    assert_eq!("ADD".try_into(), Ok(Code::ADD));
    assert_eq!("AND".try_into(), Ok(Code::AND));
    assert_eq!("XOR".try_into() as Result<Code, ()>, Err(()));
}

#[test]
fn test_into_str() {
    let s: &str = Code::ADD.into();
    assert_eq!(s, "ADD");
    let s: &str = Code::OR.into();
    assert_eq!(s, "OR");
}
