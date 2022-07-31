pub type PtrOfArgLayer = core::Ptr<crate::dnn::ArgLayer>;

ptr_extern! { crate::dnn::ArgLayer,
	cv_PtrOfArgLayer_delete, cv_PtrOfArgLayer_get_inner_ptr, cv_PtrOfArgLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::ArgLayer, cv_PtrOfArgLayer_new }

impl PtrOfArgLayer {
	#[inline] pub fn as_raw_PtrOfArgLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfArgLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ArgLayerTraitConst for PtrOfArgLayer {
	#[inline] fn as_raw_ArgLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ArgLayerTrait for PtrOfArgLayer {
	#[inline] fn as_raw_mut_ArgLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfArgLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfArgLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfArgLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfArgLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

