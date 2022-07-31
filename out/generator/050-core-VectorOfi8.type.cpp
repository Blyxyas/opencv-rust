extern "C" {
	void cv_VectorOfi8_delete(std::vector<char>* instance) {
		delete instance;
	}

	std::vector<char>* cv_VectorOfi8_new() {
		return new std::vector<char>();
	}

	size_t cv_VectorOfi8_len(const std::vector<char>* instance) {
		return instance->size();
	}

	bool cv_VectorOfi8_is_empty(const std::vector<char>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfi8_capacity(const std::vector<char>* instance) {
		return instance->capacity();
	}

	void cv_<parameter not found>_resize(std::vector<char>* instance, size_t new_size) {
		instance->resize(new_size)
	}

	void cv_VectorOfi8_shrink_to_fit(std::vector<char>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfi8_reserve(std::vector<char>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfi8_remove(std::vector<char>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfi8_swap(std::vector<char>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfi8_clear(std::vector<char>* instance) {
		instance->clear();
	}

	void cv_VectorOfi8_push(std::vector<char>* instance, char val) {
		instance->push_back(val);
	}

	void cv_VectorOfi8_insert(std::vector<char>* instance, size_t index, char val) {
		instance->insert(instance->begin() + index, val);
	}

	void cv_VectorOfi8_get(const std::vector<char>* instance, size_t index, char* ocvrs_return) {
		*ocvrs_return = (*instance)[index];
	}

	void cv_VectorOfi8_set(std::vector<char>* instance, size_t index, char val) {
		(*instance)[index] = val;
	}

	const char* cv_VectorOfi8_data(const std::vector<char>* instance) {
		return instance->data();
	}
	
	char* cv_VectorOfi8_data_mut(std::vector<char>* instance) {
		return instance->data();
	}
	
	std::vector<char>* cv_VectorOfi8_clone(const std::vector<char>* instance) {
		return new std::vector<char>(*instance);
	}
	
	std::vector<char>* cv_VectorOfi8_from_slice(const char* data, size_t len) {
		return new std::vector<char>(data, data + len);
	}
	void cv_VectorOfi8_input_array(std::vector<char>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			Ok(new cv::_InputArray(*instance), ocvrs_return);
		} OCVRS_CATCH(Result<cv::_InputArray*>)
	}
	
	void cv_VectorOfi8_output_array(std::vector<char>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			Ok(new cv::_OutputArray(*instance), ocvrs_return);
		} OCVRS_CATCH(Result<cv::_OutputArray*>)
	}
	
	void cv_VectorOfi8_input_output_array(std::vector<char>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			Ok(new cv::_InputOutputArray(*instance), ocvrs_return);
		} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
	}
	
}


