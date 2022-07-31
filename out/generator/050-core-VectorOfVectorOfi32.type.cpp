extern "C" {
	void cv_VectorOfVectorOfi32_delete(std::vector<std::vector<int>>* instance) {
		delete instance;
	}

	std::vector<std::vector<int>>* cv_VectorOfVectorOfi32_new() {
		return new std::vector<std::vector<int>>();
	}

	size_t cv_VectorOfVectorOfi32_len(const std::vector<std::vector<int>>* instance) {
		return instance->size();
	}

	bool cv_VectorOfVectorOfi32_is_empty(const std::vector<std::vector<int>>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfVectorOfi32_capacity(const std::vector<std::vector<int>>* instance) {
		return instance->capacity();
	}

	void cv_<parameter not found>_resize(std::vector<std::vector<int>>* instance, size_t new_size) {
		instance->resize(new_size)
	}

	void cv_VectorOfVectorOfi32_shrink_to_fit(std::vector<std::vector<int>>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfVectorOfi32_reserve(std::vector<std::vector<int>>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfVectorOfi32_remove(std::vector<std::vector<int>>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfVectorOfi32_swap(std::vector<std::vector<int>>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfVectorOfi32_clear(std::vector<std::vector<int>>* instance) {
		instance->clear();
	}

	void cv_VectorOfVectorOfi32_push(std::vector<std::vector<int>>* instance, std::vector<int>* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfVectorOfi32_insert(std::vector<std::vector<int>>* instance, size_t index, std::vector<int>* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfVectorOfi32_get(const std::vector<std::vector<int>>* instance, size_t index, std::vector<int>** ocvrs_return) {
		*ocvrs_return = new std::vector<int>((*instance)[index]);
	}

	void cv_VectorOfVectorOfi32_set(std::vector<std::vector<int>>* instance, size_t index, std::vector<int>* val) {
		(*instance)[index] = *val;
	}

	void cv_VectorOfVectorOfi32_input_array(std::vector<std::vector<int>>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			Ok(new cv::_InputArray(*instance), ocvrs_return);
		} OCVRS_CATCH(Result<cv::_InputArray*>)
	}
	
	void cv_VectorOfVectorOfi32_output_array(std::vector<std::vector<int>>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			Ok(new cv::_OutputArray(*instance), ocvrs_return);
		} OCVRS_CATCH(Result<cv::_OutputArray*>)
	}
	
	void cv_VectorOfVectorOfi32_input_output_array(std::vector<std::vector<int>>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			Ok(new cv::_InputOutputArray(*instance), ocvrs_return);
		} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
	}
	
}


