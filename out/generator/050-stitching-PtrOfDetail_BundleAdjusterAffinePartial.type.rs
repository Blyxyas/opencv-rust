pub type PtrOfDetail_BundleAdjusterAffinePartial = core::Ptr<crate::stitching::Detail_BundleAdjusterAffinePartial>;

ptr_extern! { crate::stitching::Detail_BundleAdjusterAffinePartial,
	cv_PtrOfDetail_BundleAdjusterAffinePartial_delete, cv_PtrOfDetail_BundleAdjusterAffinePartial_get_inner_ptr, cv_PtrOfDetail_BundleAdjusterAffinePartial_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::stitching::Detail_BundleAdjusterAffinePartial, cv_PtrOfDetail_BundleAdjusterAffinePartial_new }

impl PtrOfDetail_BundleAdjusterAffinePartial {
	#[inline] pub fn as_raw_PtrOfDetail_BundleAdjusterAffinePartial(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_BundleAdjusterAffinePartial(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_BundleAdjusterAffinePartialTraitConst for PtrOfDetail_BundleAdjusterAffinePartial {
	#[inline] fn as_raw_Detail_BundleAdjusterAffinePartial(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_BundleAdjusterAffinePartialTrait for PtrOfDetail_BundleAdjusterAffinePartial {
	#[inline] fn as_raw_mut_Detail_BundleAdjusterAffinePartial(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::Detail_BundleAdjusterBaseConst for PtrOfDetail_BundleAdjusterAffinePartial {
	#[inline] fn as_raw_Detail_BundleAdjusterBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_BundleAdjusterBase for PtrOfDetail_BundleAdjusterAffinePartial {
	#[inline] fn as_raw_mut_Detail_BundleAdjusterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfDetail_BundleAdjusterAffinePartial, core::Ptr<dyn crate::stitching::Detail_BundleAdjusterBase>,
	cv_PtrOfDetail_BundleAdjusterAffinePartial_to_PtrOfDetail_BundleAdjusterBase,
}

impl crate::stitching::Detail_EstimatorConst for PtrOfDetail_BundleAdjusterAffinePartial {
	#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_Estimator for PtrOfDetail_BundleAdjusterAffinePartial {
	#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfDetail_BundleAdjusterAffinePartial, core::Ptr<dyn crate::stitching::Detail_Estimator>,
	cv_PtrOfDetail_BundleAdjusterAffinePartial_to_PtrOfDetail_Estimator,
}

