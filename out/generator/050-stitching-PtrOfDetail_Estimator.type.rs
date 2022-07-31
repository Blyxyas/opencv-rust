pub type PtrOfDetail_Estimator = core::Ptr<dyn crate::stitching::Detail_Estimator>;

ptr_extern! { dyn crate::stitching::Detail_Estimator,
	cv_PtrOfDetail_Estimator_delete, cv_PtrOfDetail_Estimator_get_inner_ptr, cv_PtrOfDetail_Estimator_get_inner_ptr_mut
}

impl PtrOfDetail_Estimator {
	#[inline] pub fn as_raw_PtrOfDetail_Estimator(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_Estimator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_EstimatorConst for PtrOfDetail_Estimator {
	#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_Estimator for PtrOfDetail_Estimator {
	#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

