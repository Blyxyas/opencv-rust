pub type PtrOfSwishLayer = core::Ptr<crate::dnn::SwishLayer>;

ptr_extern! { crate::dnn::SwishLayer,
	cv_PtrOfSwishLayer_delete, cv_PtrOfSwishLayer_get_inner_ptr, cv_PtrOfSwishLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::SwishLayer, cv_PtrOfSwishLayer_new }

impl PtrOfSwishLayer {
	#[inline] pub fn as_raw_PtrOfSwishLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSwishLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::SwishLayerTraitConst for PtrOfSwishLayer {
	#[inline] fn as_raw_SwishLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::SwishLayerTrait for PtrOfSwishLayer {
	#[inline] fn as_raw_mut_SwishLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfSwishLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfSwishLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::ActivationLayerTraitConst for PtrOfSwishLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for PtrOfSwishLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfSwishLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfSwishLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

