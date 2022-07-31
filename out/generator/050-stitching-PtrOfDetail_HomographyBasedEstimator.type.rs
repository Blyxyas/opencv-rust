pub type PtrOfDetail_HomographyBasedEstimator = core::Ptr<crate::stitching::Detail_HomographyBasedEstimator>;

ptr_extern! { crate::stitching::Detail_HomographyBasedEstimator,
	cv_PtrOfDetail_HomographyBasedEstimator_delete, cv_PtrOfDetail_HomographyBasedEstimator_get_inner_ptr, cv_PtrOfDetail_HomographyBasedEstimator_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::stitching::Detail_HomographyBasedEstimator, cv_PtrOfDetail_HomographyBasedEstimator_new }

impl PtrOfDetail_HomographyBasedEstimator {
	#[inline] pub fn as_raw_PtrOfDetail_HomographyBasedEstimator(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_HomographyBasedEstimator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_HomographyBasedEstimatorTraitConst for PtrOfDetail_HomographyBasedEstimator {
	#[inline] fn as_raw_Detail_HomographyBasedEstimator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_HomographyBasedEstimatorTrait for PtrOfDetail_HomographyBasedEstimator {
	#[inline] fn as_raw_mut_Detail_HomographyBasedEstimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::Detail_EstimatorConst for PtrOfDetail_HomographyBasedEstimator {
	#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_Estimator for PtrOfDetail_HomographyBasedEstimator {
	#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfDetail_HomographyBasedEstimator, core::Ptr<dyn crate::stitching::Detail_Estimator>,
	cv_PtrOfDetail_HomographyBasedEstimator_to_PtrOfDetail_Estimator,
}

