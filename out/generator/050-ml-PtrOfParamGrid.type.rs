pub type PtrOfParamGrid = core::Ptr<crate::ml::ParamGrid>;

ptr_extern! { crate::ml::ParamGrid,
	cv_PtrOfParamGrid_delete, cv_PtrOfParamGrid_get_inner_ptr, cv_PtrOfParamGrid_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::ml::ParamGrid, cv_PtrOfParamGrid_new }

impl PtrOfParamGrid {
	#[inline] pub fn as_raw_PtrOfParamGrid(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfParamGrid(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::ml::ParamGridTraitConst for PtrOfParamGrid {
	#[inline] fn as_raw_ParamGrid(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ml::ParamGridTrait for PtrOfParamGrid {
	#[inline] fn as_raw_mut_ParamGrid(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

