pub type PtrOfPredictCollector = core::Ptr<dyn crate::face::PredictCollector>;

ptr_extern! { dyn crate::face::PredictCollector,
	cv_PtrOfPredictCollector_delete, cv_PtrOfPredictCollector_get_inner_ptr, cv_PtrOfPredictCollector_get_inner_ptr_mut
}

impl PtrOfPredictCollector {
	#[inline] pub fn as_raw_PtrOfPredictCollector(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfPredictCollector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::face::PredictCollectorConst for PtrOfPredictCollector {
	#[inline] fn as_raw_PredictCollector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::face::PredictCollector for PtrOfPredictCollector {
	#[inline] fn as_raw_mut_PredictCollector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

