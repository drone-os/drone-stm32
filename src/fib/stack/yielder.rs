use super::{Data, StackData};
use crate::{fib::FiberState, sv::Switch};
use core::{marker::PhantomData, mem::forget};

/// A communication channel for [`FiberStack`](FiberStack).
pub struct Yielder<Sv, I, Y, R>
where
    Sv: Switch<StackData<I, Y, R>>,
    I: Send + 'static,
    Y: Send + 'static,
    R: Send + 'static,
{
    _sv: PhantomData<*const Sv>,
    _input: PhantomData<*const I>,
    _yield: PhantomData<*const Y>,
    _return: PhantomData<*const R>,
}

impl<Sv, I, Y, R> Yielder<Sv, I, Y, R>
where
    Sv: Switch<StackData<I, Y, R>>,
    I: Send + 'static,
    Y: Send + 'static,
    R: Send + 'static,
{
    /// Creates a new `Yielder`. Normally one should use the yielder provided to
    /// fiber as argument.
    ///
    /// # Safety
    ///
    /// `I` and `O` types must match the enclosing fiber.
    #[inline]
    pub unsafe fn new() -> Self {
        Self {
            _sv: PhantomData,
            _input: PhantomData,
            _yield: PhantomData,
            _return: PhantomData,
        }
    }

    /// Yields from the enclosing stackful fiber.
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn stack_yield(&self, output: Y) -> I {
        unsafe {
            let output = FiberState::Yielded(output);
            let mut data = Data { output };
            let mut data_ptr = &mut data as *mut _;
            Sv::switch_back(&mut data_ptr);
            forget(data);
            data_ptr.read().input
        }
    }
}

impl<Sv, I, Y, R> Clone for Yielder<Sv, I, Y, R>
where
    Sv: Switch<StackData<I, Y, R>>,
    I: Send + 'static,
    Y: Send + 'static,
    R: Send + 'static,
{
    fn clone(&self) -> Self {
        unsafe { Self::new() }
    }
}

impl<Sv, I, Y, R> Copy for Yielder<Sv, I, Y, R>
where
    Sv: Switch<StackData<I, Y, R>>,
    I: Send + 'static,
    Y: Send + 'static,
    R: Send + 'static,
{
}
