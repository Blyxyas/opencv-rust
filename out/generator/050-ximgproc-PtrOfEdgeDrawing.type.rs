pub type PtrOfEdgeDrawing = core::Ptr<dyn crate::ximgproc::EdgeDrawing>;

ptr_extern! { dyn crate::ximgproc::EdgeDrawing,
	cv_PtrOfEdgeDrawing_delete, cv_PtrOfEdgeDrawing_get_inner_ptr, cv_PtrOfEdgeDrawing_get_inner_ptr_mut
}

impl PtrOfEdgeDrawing {
	#[inline] pub fn as_raw_PtrOfEdgeDrawing(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfEdgeDrawing(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::ximgproc::EdgeDrawingConst for PtrOfEdgeDrawing {
	#[inline] fn as_raw_EdgeDrawing(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::EdgeDrawing for PtrOfEdgeDrawing {
	#[inline] fn as_raw_mut_EdgeDrawing(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfEdgeDrawing {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfEdgeDrawing {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

