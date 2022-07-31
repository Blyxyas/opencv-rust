pub type PtrOfMVNLayer = core::Ptr<crate::dnn::MVNLayer>;

ptr_extern! { crate::dnn::MVNLayer,
	cv_PtrOfMVNLayer_delete, cv_PtrOfMVNLayer_get_inner_ptr, cv_PtrOfMVNLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::MVNLayer, cv_PtrOfMVNLayer_new }

impl PtrOfMVNLayer {
	#[inline] pub fn as_raw_PtrOfMVNLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMVNLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::MVNLayerTraitConst for PtrOfMVNLayer {
	#[inline] fn as_raw_MVNLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::MVNLayerTrait for PtrOfMVNLayer {
	#[inline] fn as_raw_mut_MVNLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfMVNLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfMVNLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfMVNLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfMVNLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

