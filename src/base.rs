use UnsafeCastable;
use Constructable;
use Castable;

use std::any::TypeId;
use std::fmt;

pub struct Base {
    pub instance: Option<*mut UnsafeCastable>
}

impl Clone for Base {
    fn clone(&self) -> Self {
        // we don't want to copy our instance pointer
        Base { instance: None }
    }
}

impl fmt::Debug for Base {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Base {{ ... }}")
    }
}

impl UnsafeCastable for Base {
    fn init_base(&mut self, b: Option<*mut UnsafeCastable>) {
        self.instance = b;
    }

    fn ident() -> TypeId {
        TypeId::of::<Base>()
    }

    fn get_ident(&self) -> TypeId {
        Self::ident()
    }

    fn get_super(&self) -> &UnsafeCastable { self }
    fn get_base(&self) -> &Base { self }
    unsafe fn u_upcast(&self, _: TypeId) -> Option<&&Base> { None }
    unsafe fn u_downcast(&self, _: TypeId) -> Option<&&Base> { None }
}

impl Constructable for Base {
    type Super = Base;

    fn default() -> Self where Self: Sized {
        Base { instance: None }
    }

    unsafe fn inherit(_: Self::Super) -> Self {
        panic!("base cannot inherit from anything")
    }
}

impl Castable for Base {}
