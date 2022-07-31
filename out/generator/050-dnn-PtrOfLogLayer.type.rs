pub type PtrOfLogLayer = core::Ptr<crate::dnn::LogLayer>;

ptr_extern! { crate::dnn::LogLayer,
	cv_PtrOfLogLayer_delete, cv_PtrOfLogLayer_get_inner_ptr, cv_PtrOfLogLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::LogLayer, cv_PtrOfLogLayer_new }

impl PtrOfLogLayer {
	#[inline] pub fn as_raw_PtrOfLogLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfLogLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LogLayerTraitConst for PtrOfLogLayer {
	#[inline] fn as_raw_LogLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LogLayerTrait for PtrOfLogLayer {
	#[inline] fn as_raw_mut_LogLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfLogLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfLogLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::ActivationLayerTraitConst for PtrOfLogLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for PtrOfLogLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfLogLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfLogLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

