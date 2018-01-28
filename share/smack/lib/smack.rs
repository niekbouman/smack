#[cfg(verifier = "smack")]
extern {
  pub fn __VERIFIER_assert(x: i32);
  pub fn __VERIFIER_assume(x: i32);
  pub fn __VERIFIER_nondet_signed_char() -> i8;
  pub fn __VERIFIER_nondet_unsigned_char() -> u8;
  pub fn __VERIFIER_nondet_signed_short() -> i16;
  pub fn __VERIFIER_nondet_unsigned_short() -> u16;
  pub fn __VERIFIER_nondet_signed_int() -> i32;
  pub fn __VERIFIER_nondet_unsigned_int() -> u32;
  pub fn __VERIFIER_nondet_signed_long_long() -> i64;
  pub fn __VERIFIER_nondet_unsigned_long_long() -> u64;
  pub fn malloc(size: usize) -> *mut u8;
  pub fn realloc(ptr: *mut u8, new_size: usize) -> *mut u8;
  pub fn free(ptr: *mut u8);
}


#[cfg(verifier = "smack")]
#[macro_export]
macro_rules! assert {
  ( $cond:expr ) =>
    (
      unsafe { __VERIFIER_assert($cond as i32); };
    )
}

#[cfg(verifier = "smack")]
#[macro_export]
macro_rules! assert_eq {
  ( $lhs:expr, $rhs:expr ) => ( assert!($lhs == $rhs); )
}

#[cfg(verifier = "smack")]
#[macro_export]
macro_rules! assert_neq {
  ( $lhs:expr, $rhs:expr ) => ( assert!($lhs != $rhs); )
}

#[macro_export]
macro_rules! assume {
  ( $cond:expr ) =>
    (
      #[cfg(verifier = "smack")]
      unsafe { __VERIFIER_assume($cond as i32); }

      #[cfg(not(verifier = "smack"))]
      ();
    )
}

#[macro_export]
macro_rules! nondet {
  ($e:expr) =>
    (
      #[cfg(verifier = "smack")]
      $e.nondet()

      #[cfg(not(verifier = "smack"))]
      $e
    )
}

pub trait NonDet {
  fn nondet(self) -> Self;
}

#[macro_export]
macro_rules! make_nondet {
  ($typ:ident, $nondet:ident) =>
    (
      impl NonDet for $typ {
        #[cfg(verifier = "smack")]
        fn nondet(self) -> Self {
          unsafe { $nondet() as Self }
        }

        #[cfg(not(verifier = "smack"))]
        fn nondet(self) -> Self {
          self
        }
      }
    );
}

/* Instantiate nondet for all integer types. */
make_nondet!(i8, __VERIFIER_nondet_signed_char);
make_nondet!(u8, __VERIFIER_nondet_unsigned_char);
make_nondet!(i16, __VERIFIER_nondet_signed_short);
make_nondet!(u16, __VERIFIER_nondet_unsigned_short);
make_nondet!(i32, __VERIFIER_nondet_signed_int);
make_nondet!(u32, __VERIFIER_nondet_unsigned_int);
make_nondet!(i64, __VERIFIER_nondet_signed_long_long);
make_nondet!(u64, __VERIFIER_nondet_unsigned_long_long);

/* Vector class.
   Based on https://doc.rust-lang.org/nomicon/vec-final.html */
use std::ptr::{self};
use std::mem;
use std::ops::{Deref, DerefMut};

#[derive(Default)]
struct PhantomData<T: Default> {
  _place_older: T,
  _padding: u64
}

struct Unique<T: Default> {
  _marker: PhantomData<T>,    // For the drop checker
  ptr: *const T,              // *const for variance
}

impl<T: Default> Unique<T> {
  pub fn new(ptr: *mut T) -> Self {
    Unique { ptr: ptr, _marker: Default::default() }
  }

  pub fn as_ptr(&self) -> *mut T {
    self.ptr as *mut T
  }
}

struct RawVec<T: Default> {
  ptr: Unique<T>,
  cap: usize,
}

impl<T: Default> RawVec<T> {
  fn new() -> Self {
    let elem_size = mem::size_of::<T>();
    let cap = 4;
    let ptr = unsafe { Unique::new(malloc(cap*elem_size) as *mut T) };
    RawVec { ptr: ptr, cap: cap }
  }

  fn grow(&mut self) {
    unsafe {
      let elem_size = mem::size_of::<T>();
      let new_cap = 2 * self.cap;
      let ptr = realloc(self.ptr.as_ptr() as *mut _, new_cap*elem_size);

      self.ptr = Unique::new(ptr as *mut _);
      self.cap = new_cap;
    }
  }
}

impl<T: Default> Drop for RawVec<T> {
  fn drop(&mut self) {
    unsafe { free(self.ptr.ptr as *mut _) };
  }
}

pub struct Vec<T: Default> {
  buf: RawVec<T>,
  len: usize,
}

impl<T: Default> Vec<T> {
  fn ptr(&self) -> *mut T { self.buf.ptr.as_ptr() }

  fn cap(&self) -> usize { self.buf.cap }

  pub fn new() -> Self {
    Vec { buf: RawVec::new(), len: 0 }
  }
  pub fn push(&mut self, elem: T) {
    if self.len == self.cap() { self.buf.grow(); }

    unsafe {
      ptr::write(self.ptr().offset(self.len as isize), elem);
    }

    // Can't fail, we'll OOM first.
    self.len += 1;
  }

  pub fn pop(&mut self) -> Option<T> {
    if self.len == 0 {
      None
    } else {
      self.len -= 1;
      unsafe {
        Some(ptr::read(self.ptr().offset(self.len as isize)))
      }
    }
  }

  pub fn insert(&mut self, index: usize, elem: T) {
    assert!(index <= self.len);
    if self.cap() == self.len { self.buf.grow(); }

    unsafe {
      if index < self.len {
        ptr::copy(self.ptr().offset(index as isize),
                  self.ptr().offset(index as isize + 1),
                  self.len - index);
      }
      ptr::write(self.ptr().offset(index as isize), elem);
      self.len += 1;
    }
  }

  pub fn remove(&mut self, index: usize) -> T {
    assert!(index < self.len);
    unsafe {
      self.len -= 1;
      let result = ptr::read(self.ptr().offset(index as isize));
      ptr::copy(self.ptr().offset(index as isize + 1),
                self.ptr().offset(index as isize),
                self.len - index);
      result
    }
  }

  pub fn into_iter(self) -> IntoIter<T> {
    unsafe {
      let iter = RawValIter::new(&self);
      let buf = ptr::read(&self.buf);
      mem::forget(self);

      IntoIter {
        iter: iter,
        _buf: buf,
      }
    }
  }
}

impl<T: Default> Drop for Vec<T> {
  fn drop(&mut self) {
    while let Some(_) = self.pop() {}
    // allocation is handled by RawVec
  }
}

impl<T: Default> Deref for Vec<T> {
  type Target = [T];
  fn deref(&self) -> &[T] {
    unsafe {
      ::std::slice::from_raw_parts(self.ptr(), self.len)
    }
  }
}

impl<T: Default> DerefMut for Vec<T> {
  fn deref_mut(&mut self) -> &mut [T] {
    unsafe {
      ::std::slice::from_raw_parts_mut(self.ptr(), self.len)
    }
  }
}

struct RawValIter<T> {
  start: *const T,
  end: *const T,
}

impl<T> RawValIter<T> {
  unsafe fn new(slice: &[T]) -> Self {
    RawValIter {
      start: slice.as_ptr(),
      end: if mem::size_of::<T>() == 0 {
        ((slice.as_ptr() as usize) + slice.len()) as *const _
      } else if slice.len() == 0 {
        slice.as_ptr()
      } else {
        slice.as_ptr().offset(slice.len() as isize)
      }
    }
  }
}

impl<T> Iterator for RawValIter<T> {
  type Item = T;
  fn next(&mut self) -> Option<T> {
    if self.start == self.end {
      None
    } else {
      unsafe {
        let result = ptr::read(self.start);
        self.start = if mem::size_of::<T>() == 0 {
          (self.start as usize + 1) as *const _
        } else {
          self.start.offset(1)
        };
        Some(result)
      }
    }
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    let elem_size = mem::size_of::<T>();
    let len = (self.end as usize - self.start as usize)
      / if elem_size == 0 { 1 } else { elem_size };
    (len, Some(len))
  }
}

impl<T> DoubleEndedIterator for RawValIter<T> {
  fn next_back(&mut self) -> Option<T> {
    if self.start == self.end {
      None
    } else {
      unsafe {
        self.end = if mem::size_of::<T>() == 0 {
          (self.end as usize - 1) as *const _
        } else {
          self.end.offset(-1)
        };
        Some(ptr::read(self.end))
      }
    }
  }
}

pub struct IntoIter<T: Default> {
  _buf: RawVec<T>, // we don't actually care about this. Just need it to live.
  iter: RawValIter<T>,
}

impl<T: Default> Iterator for IntoIter<T> {
  type Item = T;
  fn next(&mut self) -> Option<T> { self.iter.next() }
  fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}

impl<T: Default> DoubleEndedIterator for IntoIter<T> {
  fn next_back(&mut self) -> Option<T> { self.iter.next_back() }
}

impl<T: Default> Drop for IntoIter<T> {
  fn drop(&mut self) {
    for _ in &mut *self {}
  }
}

