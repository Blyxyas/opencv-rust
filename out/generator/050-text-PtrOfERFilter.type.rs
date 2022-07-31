pub type PtrOfERFilter = core::Ptr<dyn crate::text::ERFilter>;

ptr_extern! { dyn crate::text::ERFilter,
	cv_PtrOfERFilter_delete, cv_PtrOfERFilter_get_inner_ptr, cv_PtrOfERFilter_get_inner_ptr_mut
}

impl PtrOfERFilter {
	#[inline] pub fn as_raw_PtrOfERFilter(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfERFilter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::text::ERFilterConst for PtrOfERFilter {
	#[inline] fn as_raw_ERFilter(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::text::ERFilter for PtrOfERFilter {
	#[inline] fn as_raw_mut_ERFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfERFilter {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfERFilter {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

