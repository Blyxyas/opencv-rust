pub type PtrOfActivationLayer = core::Ptr<crate::dnn::ActivationLayer>;

ptr_extern! { crate::dnn::ActivationLayer,
	cv_PtrOfActivationLayer_delete, cv_PtrOfActivationLayer_get_inner_ptr, cv_PtrOfActivationLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::ActivationLayer, cv_PtrOfActivationLayer_new }

impl PtrOfActivationLayer {
	#[inline] pub fn as_raw_PtrOfActivationLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ActivationLayerTraitConst for PtrOfActivationLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for PtrOfActivationLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfActivationLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfActivationLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfActivationLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfActivationLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

