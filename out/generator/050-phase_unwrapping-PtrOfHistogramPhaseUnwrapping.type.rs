pub type PtrOfHistogramPhaseUnwrapping = core::Ptr<dyn crate::phase_unwrapping::HistogramPhaseUnwrapping>;

ptr_extern! { dyn crate::phase_unwrapping::HistogramPhaseUnwrapping,
	cv_PtrOfHistogramPhaseUnwrapping_delete, cv_PtrOfHistogramPhaseUnwrapping_get_inner_ptr, cv_PtrOfHistogramPhaseUnwrapping_get_inner_ptr_mut
}

impl PtrOfHistogramPhaseUnwrapping {
	#[inline] pub fn as_raw_PtrOfHistogramPhaseUnwrapping(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfHistogramPhaseUnwrapping(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::phase_unwrapping::HistogramPhaseUnwrappingConst for PtrOfHistogramPhaseUnwrapping {
	#[inline] fn as_raw_HistogramPhaseUnwrapping(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::phase_unwrapping::HistogramPhaseUnwrapping for PtrOfHistogramPhaseUnwrapping {
	#[inline] fn as_raw_mut_HistogramPhaseUnwrapping(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfHistogramPhaseUnwrapping {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfHistogramPhaseUnwrapping {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::phase_unwrapping::PhaseUnwrappingConst for PtrOfHistogramPhaseUnwrapping {
	#[inline] fn as_raw_PhaseUnwrapping(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::phase_unwrapping::PhaseUnwrapping for PtrOfHistogramPhaseUnwrapping {
	#[inline] fn as_raw_mut_PhaseUnwrapping(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

