pub type PtrOfMoreAccurateMotionWobbleSuppressorBase = core::Ptr<dyn crate::videostab::MoreAccurateMotionWobbleSuppressorBase>;

ptr_extern! { dyn crate::videostab::MoreAccurateMotionWobbleSuppressorBase,
	cv_PtrOfMoreAccurateMotionWobbleSuppressorBase_delete, cv_PtrOfMoreAccurateMotionWobbleSuppressorBase_get_inner_ptr, cv_PtrOfMoreAccurateMotionWobbleSuppressorBase_get_inner_ptr_mut
}

impl PtrOfMoreAccurateMotionWobbleSuppressorBase {
	#[inline] pub fn as_raw_PtrOfMoreAccurateMotionWobbleSuppressorBase(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMoreAccurateMotionWobbleSuppressorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::MoreAccurateMotionWobbleSuppressorBaseConst for PtrOfMoreAccurateMotionWobbleSuppressorBase {
	#[inline] fn as_raw_MoreAccurateMotionWobbleSuppressorBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::MoreAccurateMotionWobbleSuppressorBase for PtrOfMoreAccurateMotionWobbleSuppressorBase {
	#[inline] fn as_raw_mut_MoreAccurateMotionWobbleSuppressorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::WobbleSuppressorBaseConst for PtrOfMoreAccurateMotionWobbleSuppressorBase {
	#[inline] fn as_raw_WobbleSuppressorBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::WobbleSuppressorBase for PtrOfMoreAccurateMotionWobbleSuppressorBase {
	#[inline] fn as_raw_mut_WobbleSuppressorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfMoreAccurateMotionWobbleSuppressorBase, core::Ptr<dyn crate::videostab::WobbleSuppressorBase>,
	cv_PtrOfMoreAccurateMotionWobbleSuppressorBase_to_PtrOfWobbleSuppressorBase,
}

