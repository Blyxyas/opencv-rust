pub type PtrOfEdgeAwareInterpolator = core::Ptr<dyn crate::ximgproc::EdgeAwareInterpolator>;

ptr_extern! { dyn crate::ximgproc::EdgeAwareInterpolator,
	cv_PtrOfEdgeAwareInterpolator_delete, cv_PtrOfEdgeAwareInterpolator_get_inner_ptr, cv_PtrOfEdgeAwareInterpolator_get_inner_ptr_mut
}

impl PtrOfEdgeAwareInterpolator {
	#[inline] pub fn as_raw_PtrOfEdgeAwareInterpolator(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfEdgeAwareInterpolator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::ximgproc::EdgeAwareInterpolatorConst for PtrOfEdgeAwareInterpolator {
	#[inline] fn as_raw_EdgeAwareInterpolator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::EdgeAwareInterpolator for PtrOfEdgeAwareInterpolator {
	#[inline] fn as_raw_mut_EdgeAwareInterpolator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfEdgeAwareInterpolator {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfEdgeAwareInterpolator {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::ximgproc::SparseMatchInterpolatorConst for PtrOfEdgeAwareInterpolator {
	#[inline] fn as_raw_SparseMatchInterpolator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::SparseMatchInterpolator for PtrOfEdgeAwareInterpolator {
	#[inline] fn as_raw_mut_SparseMatchInterpolator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

