pub type VectorOfVectorOfi8 = core::Vector<core::Vector<i8>>;

impl VectorOfVectorOfi8 {
	pub fn as_raw_VectorOfVectorOfi8(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfVectorOfi8(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { core::Vector<i8>, *const c_void, *mut c_void,
	cv_VectorOfVectorOfi8_new, cv_VectorOfVectorOfi8_delete,
	cv_VectorOfVectorOfi8_len, cv_VectorOfVectorOfi8_is_empty,
	cv_VectorOfVectorOfi8_capacity, cv_VectorOfVectorOfi8_resize,
	cv_VectorOfVectorOfi8_shrink_to_fit,
	cv_VectorOfVectorOfi8_reserve, cv_VectorOfVectorOfi8_remove,
	cv_VectorOfVectorOfi8_swap, cv_VectorOfVectorOfi8_clear,
	cv_VectorOfVectorOfi8_get, cv_VectorOfVectorOfi8_set,
	cv_VectorOfVectorOfi8_push, cv_VectorOfVectorOfi8_insert,
}
vector_non_copy_or_bool! { clone core::Vector<i8> }

unsafe impl Send for core::Vector<core::Vector<i8>> {}

impl core::ToInputArray for VectorOfVectorOfi8 {
	#[inline]
	fn input_array(&self) -> Result<core::_InputArray> {
		extern "C" { fn cv_VectorOfVectorOfi8_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfVectorOfi8_input_array(self.as_raw_VectorOfVectorOfi8(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
	}
}

impl core::ToOutputArray for VectorOfVectorOfi8 {
	#[inline]
	fn output_array(&mut self) -> Result<core::_OutputArray> {
		extern "C" { fn cv_VectorOfVectorOfi8_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfVectorOfi8_output_array(self.as_raw_mut_VectorOfVectorOfi8(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
	}
}

impl core::ToInputOutputArray for VectorOfVectorOfi8 {
	#[inline]
	fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
		extern "C" { fn cv_VectorOfVectorOfi8_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfVectorOfi8_input_output_array(self.as_raw_mut_VectorOfVectorOfi8(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
	}
}

input_output_array_ref_forward! { VectorOfVectorOfi8 }

