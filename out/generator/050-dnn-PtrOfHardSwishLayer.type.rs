pub type PtrOfHardSwishLayer = core::Ptr<crate::dnn::HardSwishLayer>;

ptr_extern! { crate::dnn::HardSwishLayer,
	cv_PtrOfHardSwishLayer_delete, cv_PtrOfHardSwishLayer_get_inner_ptr, cv_PtrOfHardSwishLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::HardSwishLayer, cv_PtrOfHardSwishLayer_new }

impl PtrOfHardSwishLayer {
	#[inline] pub fn as_raw_PtrOfHardSwishLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfHardSwishLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::HardSwishLayerTraitConst for PtrOfHardSwishLayer {
	#[inline] fn as_raw_HardSwishLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::HardSwishLayerTrait for PtrOfHardSwishLayer {
	#[inline] fn as_raw_mut_HardSwishLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfHardSwishLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfHardSwishLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::ActivationLayerTraitConst for PtrOfHardSwishLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for PtrOfHardSwishLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfHardSwishLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfHardSwishLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

