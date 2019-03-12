use core::{
  ptr::write_volatile,
  task::{RawWaker, RawWakerVTable, Waker},
};

const NVIC_STIR: usize = 0xE000_EF00;

static VTABLE: RawWakerVTable = RawWakerVTable { clone, wake, drop };

pub struct WakeInt(usize);

impl WakeInt {
  #[inline]
  pub fn new(int_num: usize) -> Self {
    Self(int_num)
  }

  #[inline]
  pub fn wake(&self) {
    unsafe { write_volatile(NVIC_STIR as *mut usize, self.0) };
  }

  #[inline]
  pub fn to_waker(&self) -> Waker {
    unsafe { Waker::new_unchecked(self.to_raw_waker()) }
  }

  fn to_raw_waker(&self) -> RawWaker {
    RawWaker::new(self.0 as *const (), &VTABLE)
  }
}

unsafe fn clone(data: *const ()) -> RawWaker {
  WakeInt::new(data as usize).to_raw_waker()
}

unsafe fn wake(data: *const ()) {
  WakeInt::new(data as usize).wake();
}
