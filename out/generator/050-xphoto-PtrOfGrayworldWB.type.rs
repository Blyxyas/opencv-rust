pub type PtrOfGrayworldWB = core::Ptr<dyn crate::xphoto::GrayworldWB>;

ptr_extern! { dyn crate::xphoto::GrayworldWB,
	cv_PtrOfGrayworldWB_delete, cv_PtrOfGrayworldWB_get_inner_ptr, cv_PtrOfGrayworldWB_get_inner_ptr_mut
}

impl PtrOfGrayworldWB {
	#[inline] pub fn as_raw_PtrOfGrayworldWB(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfGrayworldWB(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::xphoto::GrayworldWBConst for PtrOfGrayworldWB {
	#[inline] fn as_raw_GrayworldWB(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xphoto::GrayworldWB for PtrOfGrayworldWB {
	#[inline] fn as_raw_mut_GrayworldWB(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfGrayworldWB {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfGrayworldWB {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::xphoto::WhiteBalancerConst for PtrOfGrayworldWB {
	#[inline] fn as_raw_WhiteBalancer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xphoto::WhiteBalancer for PtrOfGrayworldWB {
	#[inline] fn as_raw_mut_WhiteBalancer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

