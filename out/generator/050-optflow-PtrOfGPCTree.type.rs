pub type PtrOfGPCTree = core::Ptr<crate::optflow::GPCTree>;

ptr_extern! { crate::optflow::GPCTree,
	cv_PtrOfGPCTree_delete, cv_PtrOfGPCTree_get_inner_ptr, cv_PtrOfGPCTree_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::optflow::GPCTree, cv_PtrOfGPCTree_new }

impl PtrOfGPCTree {
	#[inline] pub fn as_raw_PtrOfGPCTree(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfGPCTree(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::optflow::GPCTreeTraitConst for PtrOfGPCTree {
	#[inline] fn as_raw_GPCTree(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::optflow::GPCTreeTrait for PtrOfGPCTree {
	#[inline] fn as_raw_mut_GPCTree(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfGPCTree {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfGPCTree {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

