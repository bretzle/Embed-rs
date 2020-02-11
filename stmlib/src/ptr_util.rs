use core::ops::Deref;
use core::ops::DerefMut;

#[repr(C)]
#[derive(Hash)]
pub struct Ptr<T> {
    ptr: *mut T,
}

impl<T> Ptr<T> {
    pub unsafe fn new(ptr: *mut T) -> Ptr<T> {
        debug_assert!(!ptr.is_null());
        Ptr { ptr: ptr }
    }

    pub const fn cnew(ptr: *mut T) -> Ptr<T> {
        Ptr { ptr: ptr }
    }

    #[inline(always)]
    pub fn as_pointer(&self) -> *mut T {
        self.ptr
    }
}

impl<T> Deref for Ptr<T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &T {
        unsafe { &*self.ptr }
    }
}

impl<T> DerefMut for Ptr<T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.ptr }
    }
}

impl<T> Copy for Ptr<T> {}

impl<T> Clone for Ptr<T> {
    #[inline(always)]
    fn clone(&self) -> Ptr<T> {
        *self
    }
}

impl<T> PartialEq for Ptr<T> {
    fn eq(&self, other: &Ptr<T>) -> bool {
        self.ptr == other.ptr
    }
}
