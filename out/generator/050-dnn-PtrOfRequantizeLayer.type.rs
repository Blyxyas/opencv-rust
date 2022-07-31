pub type PtrOfRequantizeLayer = core::Ptr<crate::dnn::RequantizeLayer>;

ptr_extern! { crate::dnn::RequantizeLayer,
	cv_PtrOfRequantizeLayer_delete, cv_PtrOfRequantizeLayer_get_inner_ptr, cv_PtrOfRequantizeLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::RequantizeLayer, cv_PtrOfRequantizeLayer_new }

impl PtrOfRequantizeLayer {
	#[inline] pub fn as_raw_PtrOfRequantizeLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfRequantizeLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::RequantizeLayerTraitConst for PtrOfRequantizeLayer {
	#[inline] fn as_raw_RequantizeLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::RequantizeLayerTrait for PtrOfRequantizeLayer {
	#[inline] fn as_raw_mut_RequantizeLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfRequantizeLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfRequantizeLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfRequantizeLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfRequantizeLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

