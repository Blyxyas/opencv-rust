extern "C" {
	void cv_VectorOfVectorOff32_delete(std::vector<std::vector<float>>* instance) {
		delete instance;
	}

	std::vector<std::vector<float>>* cv_VectorOfVectorOff32_new() {
		return new std::vector<std::vector<float>>();
	}

	size_t cv_VectorOfVectorOff32_len(const std::vector<std::vector<float>>* instance) {
		return instance->size();
	}

	bool cv_VectorOfVectorOff32_is_empty(const std::vector<std::vector<float>>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfVectorOff32_capacity(const std::vector<std::vector<float>>* instance) {
		return instance->capacity();
	}

	void cv_<parameter not found>_resize(std::vector<std::vector<float>>* instance, size_t new_size) {
		instance->resize(new_size)
	}

	void cv_VectorOfVectorOff32_shrink_to_fit(std::vector<std::vector<float>>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfVectorOff32_reserve(std::vector<std::vector<float>>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfVectorOff32_remove(std::vector<std::vector<float>>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfVectorOff32_swap(std::vector<std::vector<float>>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfVectorOff32_clear(std::vector<std::vector<float>>* instance) {
		instance->clear();
	}

	void cv_VectorOfVectorOff32_push(std::vector<std::vector<float>>* instance, std::vector<float>* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfVectorOff32_insert(std::vector<std::vector<float>>* instance, size_t index, std::vector<float>* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfVectorOff32_get(const std::vector<std::vector<float>>* instance, size_t index, std::vector<float>** ocvrs_return) {
		*ocvrs_return = new std::vector<float>((*instance)[index]);
	}

	void cv_VectorOfVectorOff32_set(std::vector<std::vector<float>>* instance, size_t index, std::vector<float>* val) {
		(*instance)[index] = *val;
	}

	void cv_VectorOfVectorOff32_input_array(std::vector<std::vector<float>>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			Ok(new cv::_InputArray(*instance), ocvrs_return);
		} OCVRS_CATCH(Result<cv::_InputArray*>)
	}
	
	void cv_VectorOfVectorOff32_output_array(std::vector<std::vector<float>>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			Ok(new cv::_OutputArray(*instance), ocvrs_return);
		} OCVRS_CATCH(Result<cv::_OutputArray*>)
	}
	
	void cv_VectorOfVectorOff32_input_output_array(std::vector<std::vector<float>>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			Ok(new cv::_InputOutputArray(*instance), ocvrs_return);
		} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
	}
	
}


