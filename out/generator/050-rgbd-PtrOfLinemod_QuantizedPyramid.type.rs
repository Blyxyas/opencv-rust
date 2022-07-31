pub type PtrOfLinemod_QuantizedPyramid = core::Ptr<dyn crate::rgbd::Linemod_QuantizedPyramid>;

ptr_extern! { dyn crate::rgbd::Linemod_QuantizedPyramid,
	cv_PtrOfLinemod_QuantizedPyramid_delete, cv_PtrOfLinemod_QuantizedPyramid_get_inner_ptr, cv_PtrOfLinemod_QuantizedPyramid_get_inner_ptr_mut
}

impl PtrOfLinemod_QuantizedPyramid {
	#[inline] pub fn as_raw_PtrOfLinemod_QuantizedPyramid(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfLinemod_QuantizedPyramid(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rgbd::Linemod_QuantizedPyramidConst for PtrOfLinemod_QuantizedPyramid {
	#[inline] fn as_raw_Linemod_QuantizedPyramid(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::Linemod_QuantizedPyramid for PtrOfLinemod_QuantizedPyramid {
	#[inline] fn as_raw_mut_Linemod_QuantizedPyramid(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

