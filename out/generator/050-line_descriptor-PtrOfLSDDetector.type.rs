pub type PtrOfLSDDetector = core::Ptr<crate::line_descriptor::LSDDetector>;

ptr_extern! { crate::line_descriptor::LSDDetector,
	cv_PtrOfLSDDetector_delete, cv_PtrOfLSDDetector_get_inner_ptr, cv_PtrOfLSDDetector_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::line_descriptor::LSDDetector, cv_PtrOfLSDDetector_new }

impl PtrOfLSDDetector {
	#[inline] pub fn as_raw_PtrOfLSDDetector(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfLSDDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::line_descriptor::LSDDetectorTraitConst for PtrOfLSDDetector {
	#[inline] fn as_raw_LSDDetector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::line_descriptor::LSDDetectorTrait for PtrOfLSDDetector {
	#[inline] fn as_raw_mut_LSDDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfLSDDetector {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfLSDDetector {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

