pub type PtrOfAlignMTB = core::Ptr<dyn crate::photo::AlignMTB>;

ptr_extern! { dyn crate::photo::AlignMTB,
	cv_PtrOfAlignMTB_delete, cv_PtrOfAlignMTB_get_inner_ptr, cv_PtrOfAlignMTB_get_inner_ptr_mut
}

impl PtrOfAlignMTB {
	#[inline] pub fn as_raw_PtrOfAlignMTB(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfAlignMTB(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::photo::AlignMTBConst for PtrOfAlignMTB {
	#[inline] fn as_raw_AlignMTB(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::photo::AlignMTB for PtrOfAlignMTB {
	#[inline] fn as_raw_mut_AlignMTB(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfAlignMTB {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfAlignMTB {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::photo::AlignExposuresConst for PtrOfAlignMTB {
	#[inline] fn as_raw_AlignExposures(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::photo::AlignExposures for PtrOfAlignMTB {
	#[inline] fn as_raw_mut_AlignExposures(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

