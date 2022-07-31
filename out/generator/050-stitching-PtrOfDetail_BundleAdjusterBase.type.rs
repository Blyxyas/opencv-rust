pub type PtrOfDetail_BundleAdjusterBase = core::Ptr<dyn crate::stitching::Detail_BundleAdjusterBase>;

ptr_extern! { dyn crate::stitching::Detail_BundleAdjusterBase,
	cv_PtrOfDetail_BundleAdjusterBase_delete, cv_PtrOfDetail_BundleAdjusterBase_get_inner_ptr, cv_PtrOfDetail_BundleAdjusterBase_get_inner_ptr_mut
}

impl PtrOfDetail_BundleAdjusterBase {
	#[inline] pub fn as_raw_PtrOfDetail_BundleAdjusterBase(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_BundleAdjusterBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_BundleAdjusterBaseConst for PtrOfDetail_BundleAdjusterBase {
	#[inline] fn as_raw_Detail_BundleAdjusterBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_BundleAdjusterBase for PtrOfDetail_BundleAdjusterBase {
	#[inline] fn as_raw_mut_Detail_BundleAdjusterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::Detail_EstimatorConst for PtrOfDetail_BundleAdjusterBase {
	#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_Estimator for PtrOfDetail_BundleAdjusterBase {
	#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfDetail_BundleAdjusterBase, core::Ptr<dyn crate::stitching::Detail_Estimator>,
	cv_PtrOfDetail_BundleAdjusterBase_to_PtrOfDetail_Estimator,
}

