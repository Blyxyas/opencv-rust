extern "C" {
	void cv_VectorOfVectorOfi8_delete(std::vector<std::vector<char>>* instance) {
		delete instance;
	}

	std::vector<std::vector<char>>* cv_VectorOfVectorOfi8_new() {
		return new std::vector<std::vector<char>>();
	}

	size_t cv_VectorOfVectorOfi8_len(const std::vector<std::vector<char>>* instance) {
		return instance->size();
	}

	bool cv_VectorOfVectorOfi8_is_empty(const std::vector<std::vector<char>>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfVectorOfi8_capacity(const std::vector<std::vector<char>>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfVectorOfi8_resize(std::vector<std::vector<char>>* instance, size_t new_size) {
		instance->resize(new_size);
	}

	void cv_VectorOfVectorOfi8_shrink_to_fit(std::vector<std::vector<char>>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfVectorOfi8_reserve(std::vector<std::vector<char>>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfVectorOfi8_remove(std::vector<std::vector<char>>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfVectorOfi8_swap(std::vector<std::vector<char>>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfVectorOfi8_clear(std::vector<std::vector<char>>* instance) {
		instance->clear();
	}

	void cv_VectorOfVectorOfi8_push(std::vector<std::vector<char>>* instance, std::vector<char>* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfVectorOfi8_insert(std::vector<std::vector<char>>* instance, size_t index, std::vector<char>* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfVectorOfi8_get(const std::vector<std::vector<char>>* instance, size_t index, std::vector<char>** ocvrs_return) {
		*ocvrs_return = new std::vector<char>((*instance)[index]);
	}

	void cv_VectorOfVectorOfi8_set(std::vector<std::vector<char>>* instance, size_t index, std::vector<char>* val) {
		(*instance)[index] = *val;
	}

	void cv_VectorOfVectorOfi8_input_array(std::vector<std::vector<char>>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			Ok(new cv::_InputArray(*instance), ocvrs_return);
		} OCVRS_CATCH(Result<cv::_InputArray*>)
	}
	
	void cv_VectorOfVectorOfi8_output_array(std::vector<std::vector<char>>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			Ok(new cv::_OutputArray(*instance), ocvrs_return);
		} OCVRS_CATCH(Result<cv::_OutputArray*>)
	}
	
	void cv_VectorOfVectorOfi8_input_output_array(std::vector<std::vector<char>>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			Ok(new cv::_InputOutputArray(*instance), ocvrs_return);
		} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
	}
	
}


