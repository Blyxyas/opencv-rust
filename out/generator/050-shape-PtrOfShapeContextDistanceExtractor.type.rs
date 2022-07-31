pub type PtrOfShapeContextDistanceExtractor = core::Ptr<dyn crate::shape::ShapeContextDistanceExtractor>;

ptr_extern! { dyn crate::shape::ShapeContextDistanceExtractor,
	cv_PtrOfShapeContextDistanceExtractor_delete, cv_PtrOfShapeContextDistanceExtractor_get_inner_ptr, cv_PtrOfShapeContextDistanceExtractor_get_inner_ptr_mut
}

impl PtrOfShapeContextDistanceExtractor {
	#[inline] pub fn as_raw_PtrOfShapeContextDistanceExtractor(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfShapeContextDistanceExtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::shape::ShapeContextDistanceExtractorConst for PtrOfShapeContextDistanceExtractor {
	#[inline] fn as_raw_ShapeContextDistanceExtractor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::shape::ShapeContextDistanceExtractor for PtrOfShapeContextDistanceExtractor {
	#[inline] fn as_raw_mut_ShapeContextDistanceExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfShapeContextDistanceExtractor {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfShapeContextDistanceExtractor {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::shape::ShapeDistanceExtractorConst for PtrOfShapeContextDistanceExtractor {
	#[inline] fn as_raw_ShapeDistanceExtractor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::shape::ShapeDistanceExtractor for PtrOfShapeContextDistanceExtractor {
	#[inline] fn as_raw_mut_ShapeDistanceExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

