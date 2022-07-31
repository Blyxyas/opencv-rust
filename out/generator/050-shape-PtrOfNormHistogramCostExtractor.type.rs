pub type PtrOfNormHistogramCostExtractor = core::Ptr<dyn crate::shape::NormHistogramCostExtractor>;

ptr_extern! { dyn crate::shape::NormHistogramCostExtractor,
	cv_PtrOfNormHistogramCostExtractor_delete, cv_PtrOfNormHistogramCostExtractor_get_inner_ptr, cv_PtrOfNormHistogramCostExtractor_get_inner_ptr_mut
}

impl PtrOfNormHistogramCostExtractor {
	#[inline] pub fn as_raw_PtrOfNormHistogramCostExtractor(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfNormHistogramCostExtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::shape::NormHistogramCostExtractorConst for PtrOfNormHistogramCostExtractor {
	#[inline] fn as_raw_NormHistogramCostExtractor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::shape::NormHistogramCostExtractor for PtrOfNormHistogramCostExtractor {
	#[inline] fn as_raw_mut_NormHistogramCostExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfNormHistogramCostExtractor {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfNormHistogramCostExtractor {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::shape::HistogramCostExtractorConst for PtrOfNormHistogramCostExtractor {
	#[inline] fn as_raw_HistogramCostExtractor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::shape::HistogramCostExtractor for PtrOfNormHistogramCostExtractor {
	#[inline] fn as_raw_mut_HistogramCostExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfNormHistogramCostExtractor, core::Ptr<dyn crate::shape::HistogramCostExtractor>,
	cv_PtrOfNormHistogramCostExtractor_to_PtrOfHistogramCostExtractor,
}

