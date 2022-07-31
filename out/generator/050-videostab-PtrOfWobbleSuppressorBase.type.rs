pub type PtrOfWobbleSuppressorBase = core::Ptr<dyn crate::videostab::WobbleSuppressorBase>;

ptr_extern! { dyn crate::videostab::WobbleSuppressorBase,
	cv_PtrOfWobbleSuppressorBase_delete, cv_PtrOfWobbleSuppressorBase_get_inner_ptr, cv_PtrOfWobbleSuppressorBase_get_inner_ptr_mut
}

impl PtrOfWobbleSuppressorBase {
	#[inline] pub fn as_raw_PtrOfWobbleSuppressorBase(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfWobbleSuppressorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::WobbleSuppressorBaseConst for PtrOfWobbleSuppressorBase {
	#[inline] fn as_raw_WobbleSuppressorBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::WobbleSuppressorBase for PtrOfWobbleSuppressorBase {
	#[inline] fn as_raw_mut_WobbleSuppressorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

