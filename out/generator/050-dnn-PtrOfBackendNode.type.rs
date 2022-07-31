pub type PtrOfBackendNode = core::Ptr<crate::dnn::BackendNode>;

ptr_extern! { crate::dnn::BackendNode,
	cv_PtrOfBackendNode_delete, cv_PtrOfBackendNode_get_inner_ptr, cv_PtrOfBackendNode_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::BackendNode, cv_PtrOfBackendNode_new }

impl PtrOfBackendNode {
	#[inline] pub fn as_raw_PtrOfBackendNode(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBackendNode(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::BackendNodeTraitConst for PtrOfBackendNode {
	#[inline] fn as_raw_BackendNode(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::BackendNodeTrait for PtrOfBackendNode {
	#[inline] fn as_raw_mut_BackendNode(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

