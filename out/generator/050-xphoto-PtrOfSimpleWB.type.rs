pub type PtrOfSimpleWB = core::Ptr<dyn crate::xphoto::SimpleWB>;

ptr_extern! { dyn crate::xphoto::SimpleWB,
	cv_PtrOfSimpleWB_delete, cv_PtrOfSimpleWB_get_inner_ptr, cv_PtrOfSimpleWB_get_inner_ptr_mut
}

impl PtrOfSimpleWB {
	#[inline] pub fn as_raw_PtrOfSimpleWB(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSimpleWB(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::xphoto::SimpleWBConst for PtrOfSimpleWB {
	#[inline] fn as_raw_SimpleWB(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xphoto::SimpleWB for PtrOfSimpleWB {
	#[inline] fn as_raw_mut_SimpleWB(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfSimpleWB {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfSimpleWB {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::xphoto::WhiteBalancerConst for PtrOfSimpleWB {
	#[inline] fn as_raw_WhiteBalancer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xphoto::WhiteBalancer for PtrOfSimpleWB {
	#[inline] fn as_raw_mut_WhiteBalancer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

