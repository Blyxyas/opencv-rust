pub type PtrOfDetail_BundleAdjusterReproj = core::Ptr<crate::stitching::Detail_BundleAdjusterReproj>;

ptr_extern! { crate::stitching::Detail_BundleAdjusterReproj,
	cv_PtrOfDetail_BundleAdjusterReproj_delete, cv_PtrOfDetail_BundleAdjusterReproj_get_inner_ptr, cv_PtrOfDetail_BundleAdjusterReproj_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::stitching::Detail_BundleAdjusterReproj, cv_PtrOfDetail_BundleAdjusterReproj_new }

impl PtrOfDetail_BundleAdjusterReproj {
	#[inline] pub fn as_raw_PtrOfDetail_BundleAdjusterReproj(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_BundleAdjusterReproj(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_BundleAdjusterReprojTraitConst for PtrOfDetail_BundleAdjusterReproj {
	#[inline] fn as_raw_Detail_BundleAdjusterReproj(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_BundleAdjusterReprojTrait for PtrOfDetail_BundleAdjusterReproj {
	#[inline] fn as_raw_mut_Detail_BundleAdjusterReproj(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::Detail_BundleAdjusterBaseConst for PtrOfDetail_BundleAdjusterReproj {
	#[inline] fn as_raw_Detail_BundleAdjusterBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_BundleAdjusterBase for PtrOfDetail_BundleAdjusterReproj {
	#[inline] fn as_raw_mut_Detail_BundleAdjusterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfDetail_BundleAdjusterReproj, core::Ptr<dyn crate::stitching::Detail_BundleAdjusterBase>,
	cv_PtrOfDetail_BundleAdjusterReproj_to_PtrOfDetail_BundleAdjusterBase,
}

impl crate::stitching::Detail_EstimatorConst for PtrOfDetail_BundleAdjusterReproj {
	#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_Estimator for PtrOfDetail_BundleAdjusterReproj {
	#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfDetail_BundleAdjusterReproj, core::Ptr<dyn crate::stitching::Detail_Estimator>,
	cv_PtrOfDetail_BundleAdjusterReproj_to_PtrOfDetail_Estimator,
}

