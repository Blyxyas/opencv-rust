pub type PtrOfNullWobbleSuppressor = core::Ptr<crate::videostab::NullWobbleSuppressor>;

ptr_extern! { crate::videostab::NullWobbleSuppressor,
	cv_PtrOfNullWobbleSuppressor_delete, cv_PtrOfNullWobbleSuppressor_get_inner_ptr, cv_PtrOfNullWobbleSuppressor_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::videostab::NullWobbleSuppressor, cv_PtrOfNullWobbleSuppressor_new }

impl PtrOfNullWobbleSuppressor {
	#[inline] pub fn as_raw_PtrOfNullWobbleSuppressor(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfNullWobbleSuppressor(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::NullWobbleSuppressorTraitConst for PtrOfNullWobbleSuppressor {
	#[inline] fn as_raw_NullWobbleSuppressor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::NullWobbleSuppressorTrait for PtrOfNullWobbleSuppressor {
	#[inline] fn as_raw_mut_NullWobbleSuppressor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::WobbleSuppressorBaseConst for PtrOfNullWobbleSuppressor {
	#[inline] fn as_raw_WobbleSuppressorBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::WobbleSuppressorBase for PtrOfNullWobbleSuppressor {
	#[inline] fn as_raw_mut_WobbleSuppressorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfNullWobbleSuppressor, core::Ptr<dyn crate::videostab::WobbleSuppressorBase>,
	cv_PtrOfNullWobbleSuppressor_to_PtrOfWobbleSuppressorBase,
}

