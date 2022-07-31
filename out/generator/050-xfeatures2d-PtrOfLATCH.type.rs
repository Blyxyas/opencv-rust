pub type PtrOfLATCH = core::Ptr<crate::xfeatures2d::LATCH>;

ptr_extern! { crate::xfeatures2d::LATCH,
	cv_PtrOfLATCH_delete, cv_PtrOfLATCH_get_inner_ptr, cv_PtrOfLATCH_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::xfeatures2d::LATCH, cv_PtrOfLATCH_new }

impl PtrOfLATCH {
	#[inline] pub fn as_raw_PtrOfLATCH(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfLATCH(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::xfeatures2d::LATCHTraitConst for PtrOfLATCH {
	#[inline] fn as_raw_LATCH(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xfeatures2d::LATCHTrait for PtrOfLATCH {
	#[inline] fn as_raw_mut_LATCH(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfLATCH {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfLATCH {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::features2d::Feature2DTraitConst for PtrOfLATCH {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::Feature2DTrait for PtrOfLATCH {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfLATCH, core::Ptr<crate::features2d::Feature2D>,
	cv_PtrOfLATCH_to_PtrOfFeature2D,
}

