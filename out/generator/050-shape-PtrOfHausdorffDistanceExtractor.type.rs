pub type PtrOfHausdorffDistanceExtractor = core::Ptr<dyn crate::shape::HausdorffDistanceExtractor>;

ptr_extern! { dyn crate::shape::HausdorffDistanceExtractor,
	cv_PtrOfHausdorffDistanceExtractor_delete, cv_PtrOfHausdorffDistanceExtractor_get_inner_ptr, cv_PtrOfHausdorffDistanceExtractor_get_inner_ptr_mut
}

impl PtrOfHausdorffDistanceExtractor {
	#[inline] pub fn as_raw_PtrOfHausdorffDistanceExtractor(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfHausdorffDistanceExtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::shape::HausdorffDistanceExtractorConst for PtrOfHausdorffDistanceExtractor {
	#[inline] fn as_raw_HausdorffDistanceExtractor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::shape::HausdorffDistanceExtractor for PtrOfHausdorffDistanceExtractor {
	#[inline] fn as_raw_mut_HausdorffDistanceExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfHausdorffDistanceExtractor {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfHausdorffDistanceExtractor {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::shape::ShapeDistanceExtractorConst for PtrOfHausdorffDistanceExtractor {
	#[inline] fn as_raw_ShapeDistanceExtractor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::shape::ShapeDistanceExtractor for PtrOfHausdorffDistanceExtractor {
	#[inline] fn as_raw_mut_ShapeDistanceExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

