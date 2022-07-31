extern "C" {
	void cv_VectorOff64_delete(std::vector<double>* instance) {
		delete instance;
	}

	std::vector<double>* cv_VectorOff64_new() {
		return new std::vector<double>();
	}

	size_t cv_VectorOff64_len(const std::vector<double>* instance) {
		return instance->size();
	}

	bool cv_VectorOff64_is_empty(const std::vector<double>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOff64_capacity(const std::vector<double>* instance) {
		return instance->capacity();
	}

	void cv_VectorOff64_resize(std::vector<double>* instance, size_t new_size) {
		instance->resize(new_size);
	}

	void cv_VectorOff64_shrink_to_fit(std::vector<double>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOff64_reserve(std::vector<double>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOff64_remove(std::vector<double>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOff64_swap(std::vector<double>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOff64_clear(std::vector<double>* instance) {
		instance->clear();
	}

	void cv_VectorOff64_push(std::vector<double>* instance, double val) {
		instance->push_back(val);
	}

	void cv_VectorOff64_insert(std::vector<double>* instance, size_t index, double val) {
		instance->insert(instance->begin() + index, val);
	}

	void cv_VectorOff64_get(const std::vector<double>* instance, size_t index, double* ocvrs_return) {
		*ocvrs_return = (*instance)[index];
	}

	void cv_VectorOff64_set(std::vector<double>* instance, size_t index, double val) {
		(*instance)[index] = val;
	}

	const double* cv_VectorOff64_data(const std::vector<double>* instance) {
		return instance->data();
	}
	
	double* cv_VectorOff64_data_mut(std::vector<double>* instance) {
		return instance->data();
	}
	
	std::vector<double>* cv_VectorOff64_clone(const std::vector<double>* instance) {
		return new std::vector<double>(*instance);
	}
	
	std::vector<double>* cv_VectorOff64_from_slice(const double* data, size_t len) {
		return new std::vector<double>(data, data + len);
	}
	void cv_VectorOff64_input_array(std::vector<double>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			Ok(new cv::_InputArray(*instance), ocvrs_return);
		} OCVRS_CATCH(Result<cv::_InputArray*>)
	}
	
	void cv_VectorOff64_output_array(std::vector<double>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			Ok(new cv::_OutputArray(*instance), ocvrs_return);
		} OCVRS_CATCH(Result<cv::_OutputArray*>)
	}
	
	void cv_VectorOff64_input_output_array(std::vector<double>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			Ok(new cv::_InputOutputArray(*instance), ocvrs_return);
		} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
	}
	
}


