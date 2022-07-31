pub type PtrOfDetail_NoBundleAdjuster = core::Ptr<crate::stitching::Detail_NoBundleAdjuster>;

ptr_extern! { crate::stitching::Detail_NoBundleAdjuster,
	cv_PtrOfDetail_NoBundleAdjuster_delete, cv_PtrOfDetail_NoBundleAdjuster_get_inner_ptr, cv_PtrOfDetail_NoBundleAdjuster_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::stitching::Detail_NoBundleAdjuster, cv_PtrOfDetail_NoBundleAdjuster_new }

impl PtrOfDetail_NoBundleAdjuster {
	#[inline] pub fn as_raw_PtrOfDetail_NoBundleAdjuster(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_NoBundleAdjuster(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_NoBundleAdjusterTraitConst for PtrOfDetail_NoBundleAdjuster {
	#[inline] fn as_raw_Detail_NoBundleAdjuster(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_NoBundleAdjusterTrait for PtrOfDetail_NoBundleAdjuster {
	#[inline] fn as_raw_mut_Detail_NoBundleAdjuster(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::Detail_BundleAdjusterBaseConst for PtrOfDetail_NoBundleAdjuster {
	#[inline] fn as_raw_Detail_BundleAdjusterBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_BundleAdjusterBase for PtrOfDetail_NoBundleAdjuster {
	#[inline] fn as_raw_mut_Detail_BundleAdjusterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfDetail_NoBundleAdjuster, core::Ptr<dyn crate::stitching::Detail_BundleAdjusterBase>,
	cv_PtrOfDetail_NoBundleAdjuster_to_PtrOfDetail_BundleAdjusterBase,
}

impl crate::stitching::Detail_EstimatorConst for PtrOfDetail_NoBundleAdjuster {
	#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_Estimator for PtrOfDetail_NoBundleAdjuster {
	#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfDetail_NoBundleAdjuster, core::Ptr<dyn crate::stitching::Detail_Estimator>,
	cv_PtrOfDetail_NoBundleAdjuster_to_PtrOfDetail_Estimator,
}

