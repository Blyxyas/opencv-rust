pub type PtrOfSinhLayer = core::Ptr<crate::dnn::SinhLayer>;

ptr_extern! { crate::dnn::SinhLayer,
	cv_PtrOfSinhLayer_delete, cv_PtrOfSinhLayer_get_inner_ptr, cv_PtrOfSinhLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::SinhLayer, cv_PtrOfSinhLayer_new }

impl PtrOfSinhLayer {
	#[inline] pub fn as_raw_PtrOfSinhLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSinhLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::SinhLayerTraitConst for PtrOfSinhLayer {
	#[inline] fn as_raw_SinhLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::SinhLayerTrait for PtrOfSinhLayer {
	#[inline] fn as_raw_mut_SinhLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfSinhLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfSinhLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::ActivationLayerTraitConst for PtrOfSinhLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for PtrOfSinhLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfSinhLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfSinhLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

