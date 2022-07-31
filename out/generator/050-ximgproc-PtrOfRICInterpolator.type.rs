pub type PtrOfRICInterpolator = core::Ptr<dyn crate::ximgproc::RICInterpolator>;

ptr_extern! { dyn crate::ximgproc::RICInterpolator,
	cv_PtrOfRICInterpolator_delete, cv_PtrOfRICInterpolator_get_inner_ptr, cv_PtrOfRICInterpolator_get_inner_ptr_mut
}

impl PtrOfRICInterpolator {
	#[inline] pub fn as_raw_PtrOfRICInterpolator(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfRICInterpolator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::ximgproc::RICInterpolatorConst for PtrOfRICInterpolator {
	#[inline] fn as_raw_RICInterpolator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::RICInterpolator for PtrOfRICInterpolator {
	#[inline] fn as_raw_mut_RICInterpolator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfRICInterpolator {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfRICInterpolator {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::ximgproc::SparseMatchInterpolatorConst for PtrOfRICInterpolator {
	#[inline] fn as_raw_SparseMatchInterpolator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::SparseMatchInterpolator for PtrOfRICInterpolator {
	#[inline] fn as_raw_mut_SparseMatchInterpolator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

