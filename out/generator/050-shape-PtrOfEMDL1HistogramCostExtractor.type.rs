pub type PtrOfEMDL1HistogramCostExtractor = core::Ptr<dyn crate::shape::EMDL1HistogramCostExtractor>;

ptr_extern! { dyn crate::shape::EMDL1HistogramCostExtractor,
	cv_PtrOfEMDL1HistogramCostExtractor_delete, cv_PtrOfEMDL1HistogramCostExtractor_get_inner_ptr, cv_PtrOfEMDL1HistogramCostExtractor_get_inner_ptr_mut
}

impl PtrOfEMDL1HistogramCostExtractor {
	#[inline] pub fn as_raw_PtrOfEMDL1HistogramCostExtractor(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfEMDL1HistogramCostExtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::shape::EMDL1HistogramCostExtractorConst for PtrOfEMDL1HistogramCostExtractor {
	#[inline] fn as_raw_EMDL1HistogramCostExtractor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::shape::EMDL1HistogramCostExtractor for PtrOfEMDL1HistogramCostExtractor {
	#[inline] fn as_raw_mut_EMDL1HistogramCostExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfEMDL1HistogramCostExtractor {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfEMDL1HistogramCostExtractor {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::shape::HistogramCostExtractorConst for PtrOfEMDL1HistogramCostExtractor {
	#[inline] fn as_raw_HistogramCostExtractor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::shape::HistogramCostExtractor for PtrOfEMDL1HistogramCostExtractor {
	#[inline] fn as_raw_mut_HistogramCostExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfEMDL1HistogramCostExtractor, core::Ptr<dyn crate::shape::HistogramCostExtractor>,
	cv_PtrOfEMDL1HistogramCostExtractor_to_PtrOfHistogramCostExtractor,
}

