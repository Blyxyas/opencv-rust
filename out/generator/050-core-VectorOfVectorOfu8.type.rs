pub type VectorOfVectorOfu8 = core::Vector<core::Vector<u8>>;

impl VectorOfVectorOfu8 {
	pub fn as_raw_VectorOfVectorOfu8(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfVectorOfu8(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { core::Vector<u8>, *const c_void, *mut c_void,
	cv_VectorOfVectorOfu8_new, cv_VectorOfVectorOfu8_delete,
	cv_VectorOfVectorOfu8_len, cv_VectorOfVectorOfu8_is_empty,
	cv_VectorOfVectorOfu8_capacity, cv_VectorOfVectorOfu8_resize,
	cv_VectorOfVectorOfu8_shrink_to_fit,
	cv_VectorOfVectorOfu8_reserve, cv_VectorOfVectorOfu8_remove,
	cv_VectorOfVectorOfu8_swap, cv_VectorOfVectorOfu8_clear,
	cv_VectorOfVectorOfu8_get, cv_VectorOfVectorOfu8_set,
	cv_VectorOfVectorOfu8_push, cv_VectorOfVectorOfu8_insert,
}
vector_non_copy_or_bool! { clone core::Vector<u8> }

unsafe impl Send for core::Vector<core::Vector<u8>> {}

impl core::ToInputArray for VectorOfVectorOfu8 {
	#[inline]
	fn input_array(&self) -> Result<core::_InputArray> {
		extern "C" { fn cv_VectorOfVectorOfu8_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfVectorOfu8_input_array(self.as_raw_VectorOfVectorOfu8(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
	}
}

impl core::ToOutputArray for VectorOfVectorOfu8 {
	#[inline]
	fn output_array(&mut self) -> Result<core::_OutputArray> {
		extern "C" { fn cv_VectorOfVectorOfu8_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfVectorOfu8_output_array(self.as_raw_mut_VectorOfVectorOfu8(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
	}
}

impl core::ToInputOutputArray for VectorOfVectorOfu8 {
	#[inline]
	fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
		extern "C" { fn cv_VectorOfVectorOfu8_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfVectorOfu8_input_output_array(self.as_raw_mut_VectorOfVectorOfu8(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
	}
}

input_output_array_ref_forward! { VectorOfVectorOfu8 }

