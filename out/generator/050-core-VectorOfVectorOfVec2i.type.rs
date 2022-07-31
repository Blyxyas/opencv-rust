pub type VectorOfVectorOfVec2i = core::Vector<core::Vector<core::Vec2i>>;

impl VectorOfVectorOfVec2i {
	pub fn as_raw_VectorOfVectorOfVec2i(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfVectorOfVec2i(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { core::Vector<core::Vec2i>, *const c_void, *mut c_void,
	cv_VectorOfVectorOfVec2i_new, cv_VectorOfVectorOfVec2i_delete,
	cv_VectorOfVectorOfVec2i_len, cv_VectorOfVectorOfVec2i_is_empty,
	cv_VectorOfVectorOfVec2i_capacity, cv_VectorOfVectorOfVec2i_resize,
	cv_VectorOfVectorOfVec2i_shrink_to_fit,
	cv_VectorOfVectorOfVec2i_reserve, cv_VectorOfVectorOfVec2i_remove,
	cv_VectorOfVectorOfVec2i_swap, cv_VectorOfVectorOfVec2i_clear,
	cv_VectorOfVectorOfVec2i_get, cv_VectorOfVectorOfVec2i_set,
	cv_VectorOfVectorOfVec2i_push, cv_VectorOfVectorOfVec2i_insert,
}
vector_non_copy_or_bool! { clone core::Vector<core::Vec2i> }

unsafe impl Send for core::Vector<core::Vector<core::Vec2i>> {}

impl core::ToInputArray for VectorOfVectorOfVec2i {
	#[inline]
	fn input_array(&self) -> Result<core::_InputArray> {
		extern "C" { fn cv_VectorOfVectorOfVec2i_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfVectorOfVec2i_input_array(self.as_raw_VectorOfVectorOfVec2i(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
	}
}

impl core::ToOutputArray for VectorOfVectorOfVec2i {
	#[inline]
	fn output_array(&mut self) -> Result<core::_OutputArray> {
		extern "C" { fn cv_VectorOfVectorOfVec2i_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfVectorOfVec2i_output_array(self.as_raw_mut_VectorOfVectorOfVec2i(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
	}
}

impl core::ToInputOutputArray for VectorOfVectorOfVec2i {
	#[inline]
	fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
		extern "C" { fn cv_VectorOfVectorOfVec2i_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfVectorOfVec2i_input_output_array(self.as_raw_mut_VectorOfVectorOfVec2i(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
	}
}

input_output_array_ref_forward! { VectorOfVectorOfVec2i }

