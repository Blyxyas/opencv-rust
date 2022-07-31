pub type PtrOfAtanhLayer = core::Ptr<crate::dnn::AtanhLayer>;

ptr_extern! { crate::dnn::AtanhLayer,
	cv_PtrOfAtanhLayer_delete, cv_PtrOfAtanhLayer_get_inner_ptr, cv_PtrOfAtanhLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::AtanhLayer, cv_PtrOfAtanhLayer_new }

impl PtrOfAtanhLayer {
	#[inline] pub fn as_raw_PtrOfAtanhLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfAtanhLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::AtanhLayerTraitConst for PtrOfAtanhLayer {
	#[inline] fn as_raw_AtanhLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::AtanhLayerTrait for PtrOfAtanhLayer {
	#[inline] fn as_raw_mut_AtanhLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfAtanhLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfAtanhLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::ActivationLayerTraitConst for PtrOfAtanhLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for PtrOfAtanhLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfAtanhLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfAtanhLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

