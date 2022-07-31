pub type VectorOfVectorOff32 = core::Vector<core::Vector<f32>>;

impl VectorOfVectorOff32 {
	pub fn as_raw_VectorOfVectorOff32(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfVectorOff32(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { core::Vector<f32>, *const c_void, *mut c_void,
	cv_VectorOfVectorOff32_new, cv_VectorOfVectorOff32_delete,
	cv_VectorOfVectorOff32_len, cv_VectorOfVectorOff32_is_empty,
	cv_VectorOfVectorOff32_capacity, cv_VectorOfVectorOff32_resize,
	cv_VectorOfVectorOff32_shrink_to_fit,
	cv_VectorOfVectorOff32_reserve, cv_VectorOfVectorOff32_remove,
	cv_VectorOfVectorOff32_swap, cv_VectorOfVectorOff32_clear,
	cv_VectorOfVectorOff32_get, cv_VectorOfVectorOff32_set,
	cv_VectorOfVectorOff32_push, cv_VectorOfVectorOff32_insert,
}
vector_non_copy_or_bool! { clone core::Vector<f32> }

unsafe impl Send for core::Vector<core::Vector<f32>> {}

impl core::ToInputArray for VectorOfVectorOff32 {
	#[inline]
	fn input_array(&self) -> Result<core::_InputArray> {
		extern "C" { fn cv_VectorOfVectorOff32_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfVectorOff32_input_array(self.as_raw_VectorOfVectorOff32(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
	}
}

impl core::ToOutputArray for VectorOfVectorOff32 {
	#[inline]
	fn output_array(&mut self) -> Result<core::_OutputArray> {
		extern "C" { fn cv_VectorOfVectorOff32_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfVectorOff32_output_array(self.as_raw_mut_VectorOfVectorOff32(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
	}
}

impl core::ToInputOutputArray for VectorOfVectorOff32 {
	#[inline]
	fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
		extern "C" { fn cv_VectorOfVectorOff32_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfVectorOff32_input_output_array(self.as_raw_mut_VectorOfVectorOff32(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
	}
}

input_output_array_ref_forward! { VectorOfVectorOff32 }

