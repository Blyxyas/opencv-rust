pub type PtrOfAtanLayer = core::Ptr<crate::dnn::AtanLayer>;

ptr_extern! { crate::dnn::AtanLayer,
	cv_PtrOfAtanLayer_delete, cv_PtrOfAtanLayer_get_inner_ptr, cv_PtrOfAtanLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::AtanLayer, cv_PtrOfAtanLayer_new }

impl PtrOfAtanLayer {
	#[inline] pub fn as_raw_PtrOfAtanLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfAtanLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::AtanLayerTraitConst for PtrOfAtanLayer {
	#[inline] fn as_raw_AtanLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::AtanLayerTrait for PtrOfAtanLayer {
	#[inline] fn as_raw_mut_AtanLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfAtanLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfAtanLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::ActivationLayerTraitConst for PtrOfAtanLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for PtrOfAtanLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfAtanLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfAtanLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

