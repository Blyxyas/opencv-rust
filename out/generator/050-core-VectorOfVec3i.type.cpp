extern "C" {
	void cv_VectorOfVec3i_delete(std::vector<cv::Vec3i>* instance) {
		delete instance;
	}

	std::vector<cv::Vec3i>* cv_VectorOfVec3i_new() {
		return new std::vector<cv::Vec3i>();
	}

	size_t cv_VectorOfVec3i_len(const std::vector<cv::Vec3i>* instance) {
		return instance->size();
	}

	bool cv_VectorOfVec3i_is_empty(const std::vector<cv::Vec3i>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfVec3i_capacity(const std::vector<cv::Vec3i>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfVec3i_resize(std::vector<cv::Vec3i>* instance, size_t new_size) {
		instance->resize(new_size);
	}

	void cv_VectorOfVec3i_shrink_to_fit(std::vector<cv::Vec3i>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfVec3i_reserve(std::vector<cv::Vec3i>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfVec3i_remove(std::vector<cv::Vec3i>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfVec3i_swap(std::vector<cv::Vec3i>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfVec3i_clear(std::vector<cv::Vec3i>* instance) {
		instance->clear();
	}

	void cv_VectorOfVec3i_push(std::vector<cv::Vec3i>* instance, cv::Vec3i* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfVec3i_insert(std::vector<cv::Vec3i>* instance, size_t index, cv::Vec3i* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfVec3i_get(const std::vector<cv::Vec3i>* instance, size_t index, cv::Vec3i* ocvrs_return) {
		*ocvrs_return = (*instance)[index];
	}

	void cv_VectorOfVec3i_set(std::vector<cv::Vec3i>* instance, size_t index, cv::Vec3i* val) {
		(*instance)[index] = *val;
	}

	const cv::Vec3i* cv_VectorOfVec3i_data(const std::vector<cv::Vec3i>* instance) {
		return instance->data();
	}
	
	cv::Vec3i* cv_VectorOfVec3i_data_mut(std::vector<cv::Vec3i>* instance) {
		return instance->data();
	}
	
	std::vector<cv::Vec3i>* cv_VectorOfVec3i_clone(const std::vector<cv::Vec3i>* instance) {
		return new std::vector<cv::Vec3i>(*instance);
	}
	
	std::vector<cv::Vec3i>* cv_VectorOfVec3i_from_slice(const cv::Vec3i* data, size_t len) {
		return new std::vector<cv::Vec3i>(data, data + len);
	}
	void cv_VectorOfVec3i_input_array(std::vector<cv::Vec3i>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			Ok(new cv::_InputArray(*instance), ocvrs_return);
		} OCVRS_CATCH(Result<cv::_InputArray*>)
	}
	
	void cv_VectorOfVec3i_output_array(std::vector<cv::Vec3i>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			Ok(new cv::_OutputArray(*instance), ocvrs_return);
		} OCVRS_CATCH(Result<cv::_OutputArray*>)
	}
	
	void cv_VectorOfVec3i_input_output_array(std::vector<cv::Vec3i>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			Ok(new cv::_InputOutputArray(*instance), ocvrs_return);
		} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
	}
	
}


