use std::{any::Any, fmt::Debug, ops::Deref, rc::Rc};

pub mod number;
pub mod operator;
pub mod identifier;
pub mod string;
pub mod comment;
pub mod singleton_token;
pub mod keyword;


pub type TokenBox = ComplexBox<dyn Token>;

pub trait Token: Debug + Any + AsAny + TokenEq {}

pub trait TokenEq {
    fn eq(&self, other: &dyn Token) -> bool;
}

impl<T: Token + PartialEq> TokenEq for T {
    fn eq(&self, other: &dyn Token) -> bool {
        if let Some(other) = other.as_any().downcast_ref() {
            self == other
        }
        else {
            false
        }
    }
}

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
    fn into_any(self: Rc<Self>) -> Rc<dyn Any>;
}

impl<T: 'static + Token> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }
}


#[derive(Debug)]
pub enum ComplexBox<T: 'static + ?Sized> {
    Rc(Rc<T>),
    Ref(&'static T),
}

impl<T: AsAny + ?Sized> ComplexBox<T> {
    pub fn downcast<U>(&self) -> Option<ComplexBox<U>> {
        match self {
            ComplexBox::Rc(r) => {
                r.clone().into_any()
                    .downcast::<U>()
                    .ok()
                    .map(|c| ComplexBox::Rc(c))
            }
            ComplexBox::Ref(r) => {
                (*r).as_any()
                    .downcast_ref::<U>()
                    .map(|r| ComplexBox::Ref(r))
            }
        }
    }

    pub fn is<U: 'static>(&self) -> bool {
        self.as_any().is::<U>()
    }
}

impl<T: 'static + ?Sized> Deref for ComplexBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        match self {
            ComplexBox::Rc(b) => b.deref(),
            ComplexBox::Ref(r) => *r,
        }
    }
}

impl<T: ?Sized> Clone for ComplexBox<T> {
    fn clone(&self) -> Self {
        match self {
            ComplexBox::Rc(b) => ComplexBox::Rc(b.clone()),
            ComplexBox::Ref(r) => ComplexBox::Ref(r),
        }
    }
}
