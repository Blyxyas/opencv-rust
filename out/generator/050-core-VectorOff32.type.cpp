extern "C" {
	void cv_VectorOff32_delete(std::vector<float>* instance) {
		delete instance;
	}

	std::vector<float>* cv_VectorOff32_new() {
		return new std::vector<float>();
	}

	size_t cv_VectorOff32_len(const std::vector<float>* instance) {
		return instance->size();
	}

	bool cv_VectorOff32_is_empty(const std::vector<float>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOff32_capacity(const std::vector<float>* instance) {
		return instance->capacity();
	}

	void cv_<parameter not found>_resize(std::vector<float>* instance, size_t new_size) {
		instance->resize(new_size)
	}

	void cv_VectorOff32_shrink_to_fit(std::vector<float>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOff32_reserve(std::vector<float>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOff32_remove(std::vector<float>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOff32_swap(std::vector<float>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOff32_clear(std::vector<float>* instance) {
		instance->clear();
	}

	void cv_VectorOff32_push(std::vector<float>* instance, float val) {
		instance->push_back(val);
	}

	void cv_VectorOff32_insert(std::vector<float>* instance, size_t index, float val) {
		instance->insert(instance->begin() + index, val);
	}

	void cv_VectorOff32_get(const std::vector<float>* instance, size_t index, float* ocvrs_return) {
		*ocvrs_return = (*instance)[index];
	}

	void cv_VectorOff32_set(std::vector<float>* instance, size_t index, float val) {
		(*instance)[index] = val;
	}

	const float* cv_VectorOff32_data(const std::vector<float>* instance) {
		return instance->data();
	}
	
	float* cv_VectorOff32_data_mut(std::vector<float>* instance) {
		return instance->data();
	}
	
	std::vector<float>* cv_VectorOff32_clone(const std::vector<float>* instance) {
		return new std::vector<float>(*instance);
	}
	
	std::vector<float>* cv_VectorOff32_from_slice(const float* data, size_t len) {
		return new std::vector<float>(data, data + len);
	}
	void cv_VectorOff32_input_array(std::vector<float>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			Ok(new cv::_InputArray(*instance), ocvrs_return);
		} OCVRS_CATCH(Result<cv::_InputArray*>)
	}
	
	void cv_VectorOff32_output_array(std::vector<float>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			Ok(new cv::_OutputArray(*instance), ocvrs_return);
		} OCVRS_CATCH(Result<cv::_OutputArray*>)
	}
	
	void cv_VectorOff32_input_output_array(std::vector<float>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			Ok(new cv::_InputOutputArray(*instance), ocvrs_return);
		} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
	}
	
}


