//! libre allocator bridge for rust
//! Copyright MMXVII Kristopher Tate / connectFree Corporation

#![no_std]
#![feature(alloc)]
#![feature(allocator_api)]
extern crate libc;
extern crate alloc;

use libc::{c_void};

use alloc::heap::{Alloc, Layout, Excess, CannotReallocInPlace, AllocErr};


/// Defines the memory destructor handler, which is called when the reference
/// of a memory object goes down to zero
///
/// @param data Pointer to memory object
#[allow(non_camel_case_types)]
pub type mem_destroy_h =
  ::core::option::Option<unsafe extern "C" fn(data: *mut c_void)>;

extern "C" {
  #[link_name = "\u{1}_mem_alloc"]
  pub fn mem_alloc(size: libc::size_t, dh: mem_destroy_h) -> *mut c_void;
}

extern "C" {
  #[link_name = "\u{1}_mem_zalloc"]
  pub fn mem_zalloc(size: usize, dh: mem_destroy_h) -> *mut c_void;
}

extern "C" {
  #[link_name = "\u{1}_mem_realloc"]
  pub fn mem_realloc(
    data: *mut c_void,
    size: libc::size_t,
  ) -> *mut c_void;
}

extern "C" {
  #[link_name = "\u{1}_mem_deref"]
  pub fn mem_deref(data: *mut c_void) -> *mut c_void;
}

extern "C" {
  #[link_name = "\u{1}_mem_debug"]
  pub fn mem_debug();
}

/**/

pub struct LibreAlloc;

unsafe impl Alloc for LibreAlloc {
  #[inline]
  unsafe fn alloc(&mut self, layout: Layout)
    -> Result<*mut u8, AllocErr>
  {
      (&*self).alloc(layout)
  }

  #[inline]
  unsafe fn alloc_zeroed(&mut self, layout: Layout)
    -> Result<*mut u8, AllocErr>
  {
      (&*self).alloc_zeroed(layout)
  }

  #[inline]
  unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout)
  {
    (&*self).dealloc(ptr, layout)
  }

  #[inline]
  unsafe fn realloc(&mut self,
                    ptr: *mut u8,
                    old_layout: Layout,
                    new_layout: Layout)
    -> Result<*mut u8, AllocErr>
  {
    (&*self).realloc(ptr, old_layout, new_layout)
  }

  fn oom(&mut self, err: AllocErr)
    -> !
  {
    (&*self).oom(err)
  }

/*  #[inline]
  fn usable_size(&self, layout: &Layout)
    -> (usize, usize)
  {
    (&self).usable_size(layout)
  }*/

  #[inline]
  unsafe fn alloc_excess(&mut self, layout: Layout)
    -> Result<Excess, AllocErr>
  {
    (&*self).alloc_excess(layout)
  }

  #[inline]
  unsafe fn realloc_excess(&mut self,
                           ptr: *mut u8,
                           layout: Layout,
                           new_layout: Layout)
    -> Result<Excess, AllocErr>
  {
    (&*self).realloc_excess(ptr, layout, new_layout)
  }

  #[inline]
  unsafe fn grow_in_place(&mut self,
                          ptr: *mut u8,
                          layout: Layout,
                          new_layout: Layout)
    -> Result<(), CannotReallocInPlace>
  {
    (&*self).grow_in_place(ptr, layout, new_layout)
  }

  #[inline]
  unsafe fn shrink_in_place(&mut self,
                            ptr: *mut u8,
                            layout: Layout,
                            new_layout: Layout)
    -> Result<(), CannotReallocInPlace>
  {
    (&*self).shrink_in_place(ptr, layout, new_layout)
  }
}

unsafe impl<'a> Alloc for &'a LibreAlloc {
  #[inline]
  unsafe fn alloc(&mut self, layout: Layout)
    -> Result<*mut u8, AllocErr>
  {
    let ptr = mem_alloc(layout.size() as libc::size_t, None);
    if ptr.is_null() {
      Err(AllocErr::Exhausted { request: layout })
    } else {
      Ok(ptr as *mut u8)
    }
  }

  #[inline]
  unsafe fn alloc_zeroed(&mut self, layout: Layout)
      -> Result<*mut u8, AllocErr>
  {
    let ptr = mem_zalloc(layout.size() as libc::size_t, None);
    if ptr.is_null() {
      Err(AllocErr::Exhausted { request: layout })
    } else {
      Ok(ptr as *mut u8)
    }
  }

  #[inline]
  unsafe fn alloc_excess(&mut self, layout: Layout)
    -> Result<Excess, AllocErr>
  {
    let ptr = mem_alloc(layout.size() as libc::size_t, None);
    if ptr.is_null() {
      Err(AllocErr::Exhausted { request: layout })
    } else {
      Ok(Excess(ptr as *mut u8, layout.size()))
    }
  }

  #[inline]
  unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
    mem_deref(ptr as *mut c_void);
  }

  #[inline]
  unsafe fn realloc(&mut self,
                    ptr: *mut u8,
                    old_layout: Layout,
                    new_layout: Layout)
    -> Result<*mut u8, AllocErr>
  {
    if old_layout.align() != new_layout.align() {
      return Err(AllocErr::Unsupported { details: "cannot change align" })
    }
    let ptr = mem_realloc(ptr as *mut c_void, new_layout.size());
    if ptr.is_null() {
      Err(AllocErr::Exhausted { request: new_layout })
    } else {
      Ok(ptr as *mut u8)
    }
  }

  #[inline]
  unsafe fn realloc_excess(&mut self,
                    ptr: *mut u8,
                    old_layout: Layout,
                    new_layout: Layout)
    -> Result<Excess, AllocErr>
  {
    if old_layout.align() != new_layout.align() {
      return Err(AllocErr::Unsupported { details: "cannot change align" })
    }
    let ptr = mem_realloc(ptr as *mut c_void, new_layout.size());
    if ptr.is_null() {
      Err(AllocErr::Exhausted { request: new_layout })
    } else {
      Ok(Excess(ptr as *mut u8, new_layout.size()))
    }
  }

  fn oom(&mut self, err: AllocErr) -> ! {
    print_debug();
    panic!("{:?}", err);
  }

/*
  #[inline]
  fn usable_size(&self, layout: &Layout) -> (usize, usize) {
    let flags = layout_to_flags(&layout);
    unsafe {
      let max = ffi::nallocx(layout.size(), flags);
      (layout.size(), max)
    }
  }
*/

  #[inline]
  unsafe fn grow_in_place(&mut self,
                          ptr: *mut u8,
                          old_layout: Layout,
                          new_layout: Layout) -> Result<(), CannotReallocInPlace> {
      self.shrink_in_place(ptr, old_layout, new_layout)
  }

  #[inline]
  unsafe fn shrink_in_place(&mut self,
                            ptr: *mut u8,
                            old_layout: Layout,
                            new_layout: Layout)
    -> Result<(), CannotReallocInPlace>
  {
    return Err(CannotReallocInPlace)
  }
}

/**/

/// Prints libre memory debug information via `mem_debug`
pub fn print_debug() {
  unsafe { mem_debug(); }
}
