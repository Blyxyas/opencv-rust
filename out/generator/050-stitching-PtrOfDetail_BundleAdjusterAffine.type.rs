pub type PtrOfDetail_BundleAdjusterAffine = core::Ptr<crate::stitching::Detail_BundleAdjusterAffine>;

ptr_extern! { crate::stitching::Detail_BundleAdjusterAffine,
	cv_PtrOfDetail_BundleAdjusterAffine_delete, cv_PtrOfDetail_BundleAdjusterAffine_get_inner_ptr, cv_PtrOfDetail_BundleAdjusterAffine_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::stitching::Detail_BundleAdjusterAffine, cv_PtrOfDetail_BundleAdjusterAffine_new }

impl PtrOfDetail_BundleAdjusterAffine {
	#[inline] pub fn as_raw_PtrOfDetail_BundleAdjusterAffine(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_BundleAdjusterAffine(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_BundleAdjusterAffineTraitConst for PtrOfDetail_BundleAdjusterAffine {
	#[inline] fn as_raw_Detail_BundleAdjusterAffine(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_BundleAdjusterAffineTrait for PtrOfDetail_BundleAdjusterAffine {
	#[inline] fn as_raw_mut_Detail_BundleAdjusterAffine(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::Detail_BundleAdjusterBaseConst for PtrOfDetail_BundleAdjusterAffine {
	#[inline] fn as_raw_Detail_BundleAdjusterBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_BundleAdjusterBase for PtrOfDetail_BundleAdjusterAffine {
	#[inline] fn as_raw_mut_Detail_BundleAdjusterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfDetail_BundleAdjusterAffine, core::Ptr<dyn crate::stitching::Detail_BundleAdjusterBase>,
	cv_PtrOfDetail_BundleAdjusterAffine_to_PtrOfDetail_BundleAdjusterBase,
}

impl crate::stitching::Detail_EstimatorConst for PtrOfDetail_BundleAdjusterAffine {
	#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_Estimator for PtrOfDetail_BundleAdjusterAffine {
	#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfDetail_BundleAdjusterAffine, core::Ptr<dyn crate::stitching::Detail_Estimator>,
	cv_PtrOfDetail_BundleAdjusterAffine_to_PtrOfDetail_Estimator,
}

