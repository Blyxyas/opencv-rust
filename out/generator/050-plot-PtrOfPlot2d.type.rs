pub type PtrOfPlot2d = core::Ptr<dyn crate::plot::Plot2d>;

ptr_extern! { dyn crate::plot::Plot2d,
	cv_PtrOfPlot2d_delete, cv_PtrOfPlot2d_get_inner_ptr, cv_PtrOfPlot2d_get_inner_ptr_mut
}

impl PtrOfPlot2d {
	#[inline] pub fn as_raw_PtrOfPlot2d(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfPlot2d(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::plot::Plot2dConst for PtrOfPlot2d {
	#[inline] fn as_raw_Plot2d(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::plot::Plot2d for PtrOfPlot2d {
	#[inline] fn as_raw_mut_Plot2d(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfPlot2d {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfPlot2d {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

