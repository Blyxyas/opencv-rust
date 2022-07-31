pub type PtrOfTonemapDrago = core::Ptr<dyn crate::photo::TonemapDrago>;

ptr_extern! { dyn crate::photo::TonemapDrago,
	cv_PtrOfTonemapDrago_delete, cv_PtrOfTonemapDrago_get_inner_ptr, cv_PtrOfTonemapDrago_get_inner_ptr_mut
}

impl PtrOfTonemapDrago {
	#[inline] pub fn as_raw_PtrOfTonemapDrago(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTonemapDrago(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::photo::TonemapDragoConst for PtrOfTonemapDrago {
	#[inline] fn as_raw_TonemapDrago(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::photo::TonemapDrago for PtrOfTonemapDrago {
	#[inline] fn as_raw_mut_TonemapDrago(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfTonemapDrago {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfTonemapDrago {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::photo::TonemapConst for PtrOfTonemapDrago {
	#[inline] fn as_raw_Tonemap(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::photo::Tonemap for PtrOfTonemapDrago {
	#[inline] fn as_raw_mut_Tonemap(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

