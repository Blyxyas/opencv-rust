pub type VectorOfPtrOfMCC_CChecker = core::Vector<core::Ptr<dyn crate::mcc::MCC_CChecker>>;

impl VectorOfPtrOfMCC_CChecker {
	pub fn as_raw_VectorOfPtrOfMCC_CChecker(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfPtrOfMCC_CChecker(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { core::Ptr<dyn crate::mcc::MCC_CChecker>, *const c_void, *mut c_void,
	cv_VectorOfPtrOfMCC_CChecker_new, cv_VectorOfPtrOfMCC_CChecker_delete,
	cv_VectorOfPtrOfMCC_CChecker_len, cv_VectorOfPtrOfMCC_CChecker_is_empty,
	cv_VectorOfPtrOfMCC_CChecker_capacity, cv_VectorOfPtrOfMCC_CChecker_resize,
	cv_VectorOfPtrOfMCC_CChecker_shrink_to_fit,
	cv_VectorOfPtrOfMCC_CChecker_reserve, cv_VectorOfPtrOfMCC_CChecker_remove,
	cv_VectorOfPtrOfMCC_CChecker_swap, cv_VectorOfPtrOfMCC_CChecker_clear,
	cv_VectorOfPtrOfMCC_CChecker_get, cv_VectorOfPtrOfMCC_CChecker_set,
	cv_VectorOfPtrOfMCC_CChecker_push, cv_VectorOfPtrOfMCC_CChecker_insert,
}
vector_non_copy_or_bool! { core::Ptr<dyn crate::mcc::MCC_CChecker> }

unsafe impl Send for core::Vector<core::Ptr<dyn crate::mcc::MCC_CChecker>> {}

