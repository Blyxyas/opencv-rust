pub type PtrOfNotLayer = core::Ptr<crate::dnn::NotLayer>;

ptr_extern! { crate::dnn::NotLayer,
	cv_PtrOfNotLayer_delete, cv_PtrOfNotLayer_get_inner_ptr, cv_PtrOfNotLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::NotLayer, cv_PtrOfNotLayer_new }

impl PtrOfNotLayer {
	#[inline] pub fn as_raw_PtrOfNotLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfNotLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::NotLayerTraitConst for PtrOfNotLayer {
	#[inline] fn as_raw_NotLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::NotLayerTrait for PtrOfNotLayer {
	#[inline] fn as_raw_mut_NotLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfNotLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfNotLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::ActivationLayerTraitConst for PtrOfNotLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for PtrOfNotLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfNotLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfNotLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

