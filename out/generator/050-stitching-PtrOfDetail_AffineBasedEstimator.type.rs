pub type PtrOfDetail_AffineBasedEstimator = core::Ptr<crate::stitching::Detail_AffineBasedEstimator>;

ptr_extern! { crate::stitching::Detail_AffineBasedEstimator,
	cv_PtrOfDetail_AffineBasedEstimator_delete, cv_PtrOfDetail_AffineBasedEstimator_get_inner_ptr, cv_PtrOfDetail_AffineBasedEstimator_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::stitching::Detail_AffineBasedEstimator, cv_PtrOfDetail_AffineBasedEstimator_new }

impl PtrOfDetail_AffineBasedEstimator {
	#[inline] pub fn as_raw_PtrOfDetail_AffineBasedEstimator(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_AffineBasedEstimator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_AffineBasedEstimatorTraitConst for PtrOfDetail_AffineBasedEstimator {
	#[inline] fn as_raw_Detail_AffineBasedEstimator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_AffineBasedEstimatorTrait for PtrOfDetail_AffineBasedEstimator {
	#[inline] fn as_raw_mut_Detail_AffineBasedEstimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::Detail_EstimatorConst for PtrOfDetail_AffineBasedEstimator {
	#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_Estimator for PtrOfDetail_AffineBasedEstimator {
	#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfDetail_AffineBasedEstimator, core::Ptr<dyn crate::stitching::Detail_Estimator>,
	cv_PtrOfDetail_AffineBasedEstimator_to_PtrOfDetail_Estimator,
}

