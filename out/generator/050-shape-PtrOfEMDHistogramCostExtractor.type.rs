pub type PtrOfEMDHistogramCostExtractor = core::Ptr<dyn crate::shape::EMDHistogramCostExtractor>;

ptr_extern! { dyn crate::shape::EMDHistogramCostExtractor,
	cv_PtrOfEMDHistogramCostExtractor_delete, cv_PtrOfEMDHistogramCostExtractor_get_inner_ptr, cv_PtrOfEMDHistogramCostExtractor_get_inner_ptr_mut
}

impl PtrOfEMDHistogramCostExtractor {
	#[inline] pub fn as_raw_PtrOfEMDHistogramCostExtractor(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfEMDHistogramCostExtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::shape::EMDHistogramCostExtractorConst for PtrOfEMDHistogramCostExtractor {
	#[inline] fn as_raw_EMDHistogramCostExtractor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::shape::EMDHistogramCostExtractor for PtrOfEMDHistogramCostExtractor {
	#[inline] fn as_raw_mut_EMDHistogramCostExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfEMDHistogramCostExtractor {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfEMDHistogramCostExtractor {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::shape::HistogramCostExtractorConst for PtrOfEMDHistogramCostExtractor {
	#[inline] fn as_raw_HistogramCostExtractor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::shape::HistogramCostExtractor for PtrOfEMDHistogramCostExtractor {
	#[inline] fn as_raw_mut_HistogramCostExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfEMDHistogramCostExtractor, core::Ptr<dyn crate::shape::HistogramCostExtractor>,
	cv_PtrOfEMDHistogramCostExtractor_to_PtrOfHistogramCostExtractor,
}

