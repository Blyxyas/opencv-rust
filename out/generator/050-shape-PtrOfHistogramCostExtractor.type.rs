pub type PtrOfHistogramCostExtractor = core::Ptr<dyn crate::shape::HistogramCostExtractor>;

ptr_extern! { dyn crate::shape::HistogramCostExtractor,
	cv_PtrOfHistogramCostExtractor_delete, cv_PtrOfHistogramCostExtractor_get_inner_ptr, cv_PtrOfHistogramCostExtractor_get_inner_ptr_mut
}

impl PtrOfHistogramCostExtractor {
	#[inline] pub fn as_raw_PtrOfHistogramCostExtractor(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfHistogramCostExtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::shape::HistogramCostExtractorConst for PtrOfHistogramCostExtractor {
	#[inline] fn as_raw_HistogramCostExtractor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::shape::HistogramCostExtractor for PtrOfHistogramCostExtractor {
	#[inline] fn as_raw_mut_HistogramCostExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfHistogramCostExtractor {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfHistogramCostExtractor {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

