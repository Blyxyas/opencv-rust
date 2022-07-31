pub type VectorOfVectorOfi32 = core::Vector<core::Vector<i32>>;

impl VectorOfVectorOfi32 {
	pub fn as_raw_VectorOfVectorOfi32(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfVectorOfi32(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { core::Vector<i32>, *const c_void, *mut c_void,
	cv_VectorOfVectorOfi32_new, cv_VectorOfVectorOfi32_delete,
	cv_VectorOfVectorOfi32_len, cv_VectorOfVectorOfi32_is_empty,
	cv_VectorOfVectorOfi32_capacity, cv_VectorOfVectorOfi32_resize,
	cv_VectorOfVectorOfi32_shrink_to_fit,
	cv_VectorOfVectorOfi32_reserve, cv_VectorOfVectorOfi32_remove,
	cv_VectorOfVectorOfi32_swap, cv_VectorOfVectorOfi32_clear,
	cv_VectorOfVectorOfi32_get, cv_VectorOfVectorOfi32_set,
	cv_VectorOfVectorOfi32_push, cv_VectorOfVectorOfi32_insert,
}
vector_non_copy_or_bool! { clone core::Vector<i32> }

unsafe impl Send for core::Vector<core::Vector<i32>> {}

impl core::ToInputArray for VectorOfVectorOfi32 {
	#[inline]
	fn input_array(&self) -> Result<core::_InputArray> {
		extern "C" { fn cv_VectorOfVectorOfi32_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfVectorOfi32_input_array(self.as_raw_VectorOfVectorOfi32(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
	}
}

impl core::ToOutputArray for VectorOfVectorOfi32 {
	#[inline]
	fn output_array(&mut self) -> Result<core::_OutputArray> {
		extern "C" { fn cv_VectorOfVectorOfi32_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfVectorOfi32_output_array(self.as_raw_mut_VectorOfVectorOfi32(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
	}
}

impl core::ToInputOutputArray for VectorOfVectorOfi32 {
	#[inline]
	fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
		extern "C" { fn cv_VectorOfVectorOfi32_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfVectorOfi32_input_output_array(self.as_raw_mut_VectorOfVectorOfi32(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
	}
}

input_output_array_ref_forward! { VectorOfVectorOfi32 }

