pub type PtrOfEdgeBoxes = core::Ptr<dyn crate::ximgproc::EdgeBoxes>;

ptr_extern! { dyn crate::ximgproc::EdgeBoxes,
	cv_PtrOfEdgeBoxes_delete, cv_PtrOfEdgeBoxes_get_inner_ptr, cv_PtrOfEdgeBoxes_get_inner_ptr_mut
}

impl PtrOfEdgeBoxes {
	#[inline] pub fn as_raw_PtrOfEdgeBoxes(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfEdgeBoxes(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::ximgproc::EdgeBoxesConst for PtrOfEdgeBoxes {
	#[inline] fn as_raw_EdgeBoxes(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::EdgeBoxes for PtrOfEdgeBoxes {
	#[inline] fn as_raw_mut_EdgeBoxes(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfEdgeBoxes {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfEdgeBoxes {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

