pub type PtrOfChiHistogramCostExtractor = core::Ptr<dyn crate::shape::ChiHistogramCostExtractor>;

ptr_extern! { dyn crate::shape::ChiHistogramCostExtractor,
	cv_PtrOfChiHistogramCostExtractor_delete, cv_PtrOfChiHistogramCostExtractor_get_inner_ptr, cv_PtrOfChiHistogramCostExtractor_get_inner_ptr_mut
}

impl PtrOfChiHistogramCostExtractor {
	#[inline] pub fn as_raw_PtrOfChiHistogramCostExtractor(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfChiHistogramCostExtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::shape::ChiHistogramCostExtractorConst for PtrOfChiHistogramCostExtractor {
	#[inline] fn as_raw_ChiHistogramCostExtractor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::shape::ChiHistogramCostExtractor for PtrOfChiHistogramCostExtractor {
	#[inline] fn as_raw_mut_ChiHistogramCostExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfChiHistogramCostExtractor {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfChiHistogramCostExtractor {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::shape::HistogramCostExtractorConst for PtrOfChiHistogramCostExtractor {
	#[inline] fn as_raw_HistogramCostExtractor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::shape::HistogramCostExtractor for PtrOfChiHistogramCostExtractor {
	#[inline] fn as_raw_mut_HistogramCostExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfChiHistogramCostExtractor, core::Ptr<dyn crate::shape::HistogramCostExtractor>,
	cv_PtrOfChiHistogramCostExtractor_to_PtrOfHistogramCostExtractor,
}

