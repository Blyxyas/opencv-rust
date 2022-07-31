extern "C" {
	void cv_VectorOfVectorOff64_delete(std::vector<std::vector<double>>* instance) {
		delete instance;
	}

	std::vector<std::vector<double>>* cv_VectorOfVectorOff64_new() {
		return new std::vector<std::vector<double>>();
	}

	size_t cv_VectorOfVectorOff64_len(const std::vector<std::vector<double>>* instance) {
		return instance->size();
	}

	bool cv_VectorOfVectorOff64_is_empty(const std::vector<std::vector<double>>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfVectorOff64_capacity(const std::vector<std::vector<double>>* instance) {
		return instance->capacity();
	}

	void cv_<parameter not found>_resize(std::vector<std::vector<double>>* instance, size_t new_size) {
		instance->resize(new_size)
	}

	void cv_VectorOfVectorOff64_shrink_to_fit(std::vector<std::vector<double>>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfVectorOff64_reserve(std::vector<std::vector<double>>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfVectorOff64_remove(std::vector<std::vector<double>>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfVectorOff64_swap(std::vector<std::vector<double>>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfVectorOff64_clear(std::vector<std::vector<double>>* instance) {
		instance->clear();
	}

	void cv_VectorOfVectorOff64_push(std::vector<std::vector<double>>* instance, std::vector<double>* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfVectorOff64_insert(std::vector<std::vector<double>>* instance, size_t index, std::vector<double>* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfVectorOff64_get(const std::vector<std::vector<double>>* instance, size_t index, std::vector<double>** ocvrs_return) {
		*ocvrs_return = new std::vector<double>((*instance)[index]);
	}

	void cv_VectorOfVectorOff64_set(std::vector<std::vector<double>>* instance, size_t index, std::vector<double>* val) {
		(*instance)[index] = *val;
	}

	void cv_VectorOfVectorOff64_input_array(std::vector<std::vector<double>>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			Ok(new cv::_InputArray(*instance), ocvrs_return);
		} OCVRS_CATCH(Result<cv::_InputArray*>)
	}
	
	void cv_VectorOfVectorOff64_output_array(std::vector<std::vector<double>>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			Ok(new cv::_OutputArray(*instance), ocvrs_return);
		} OCVRS_CATCH(Result<cv::_OutputArray*>)
	}
	
	void cv_VectorOfVectorOff64_input_output_array(std::vector<std::vector<double>>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			Ok(new cv::_InputOutputArray(*instance), ocvrs_return);
		} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
	}
	
}


