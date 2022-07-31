pub type PtrOfDetail_BundleAdjusterRay = core::Ptr<crate::stitching::Detail_BundleAdjusterRay>;

ptr_extern! { crate::stitching::Detail_BundleAdjusterRay,
	cv_PtrOfDetail_BundleAdjusterRay_delete, cv_PtrOfDetail_BundleAdjusterRay_get_inner_ptr, cv_PtrOfDetail_BundleAdjusterRay_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::stitching::Detail_BundleAdjusterRay, cv_PtrOfDetail_BundleAdjusterRay_new }

impl PtrOfDetail_BundleAdjusterRay {
	#[inline] pub fn as_raw_PtrOfDetail_BundleAdjusterRay(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_BundleAdjusterRay(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_BundleAdjusterRayTraitConst for PtrOfDetail_BundleAdjusterRay {
	#[inline] fn as_raw_Detail_BundleAdjusterRay(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_BundleAdjusterRayTrait for PtrOfDetail_BundleAdjusterRay {
	#[inline] fn as_raw_mut_Detail_BundleAdjusterRay(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::Detail_BundleAdjusterBaseConst for PtrOfDetail_BundleAdjusterRay {
	#[inline] fn as_raw_Detail_BundleAdjusterBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_BundleAdjusterBase for PtrOfDetail_BundleAdjusterRay {
	#[inline] fn as_raw_mut_Detail_BundleAdjusterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfDetail_BundleAdjusterRay, core::Ptr<dyn crate::stitching::Detail_BundleAdjusterBase>,
	cv_PtrOfDetail_BundleAdjusterRay_to_PtrOfDetail_BundleAdjusterBase,
}

impl crate::stitching::Detail_EstimatorConst for PtrOfDetail_BundleAdjusterRay {
	#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_Estimator for PtrOfDetail_BundleAdjusterRay {
	#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfDetail_BundleAdjusterRay, core::Ptr<dyn crate::stitching::Detail_Estimator>,
	cv_PtrOfDetail_BundleAdjusterRay_to_PtrOfDetail_Estimator,
}

