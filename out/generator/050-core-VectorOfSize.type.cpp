extern "C" {
	void cv_VectorOfSize_delete(std::vector<cv::Size>* instance) {
		delete instance;
	}

	std::vector<cv::Size>* cv_VectorOfSize_new() {
		return new std::vector<cv::Size>();
	}

	size_t cv_VectorOfSize_len(const std::vector<cv::Size>* instance) {
		return instance->size();
	}

	bool cv_VectorOfSize_is_empty(const std::vector<cv::Size>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfSize_capacity(const std::vector<cv::Size>* instance) {
		return instance->capacity();
	}

	void cv_<parameter not found>_resize(std::vector<cv::Size>* instance, size_t new_size) {
		instance->resize(new_size)
	}

	void cv_VectorOfSize_shrink_to_fit(std::vector<cv::Size>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfSize_reserve(std::vector<cv::Size>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfSize_remove(std::vector<cv::Size>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfSize_swap(std::vector<cv::Size>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfSize_clear(std::vector<cv::Size>* instance) {
		instance->clear();
	}

	void cv_VectorOfSize_push(std::vector<cv::Size>* instance, cv::Size* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfSize_insert(std::vector<cv::Size>* instance, size_t index, cv::Size* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfSize_get(const std::vector<cv::Size>* instance, size_t index, cv::Size* ocvrs_return) {
		*ocvrs_return = (*instance)[index];
	}

	void cv_VectorOfSize_set(std::vector<cv::Size>* instance, size_t index, cv::Size* val) {
		(*instance)[index] = *val;
	}

	const cv::Size* cv_VectorOfSize_data(const std::vector<cv::Size>* instance) {
		return instance->data();
	}
	
	cv::Size* cv_VectorOfSize_data_mut(std::vector<cv::Size>* instance) {
		return instance->data();
	}
	
	std::vector<cv::Size>* cv_VectorOfSize_clone(const std::vector<cv::Size>* instance) {
		return new std::vector<cv::Size>(*instance);
	}
	
	std::vector<cv::Size>* cv_VectorOfSize_from_slice(const cv::Size* data, size_t len) {
		return new std::vector<cv::Size>(data, data + len);
	}
	void cv_VectorOfSize_input_array(std::vector<cv::Size>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			Ok(new cv::_InputArray(*instance), ocvrs_return);
		} OCVRS_CATCH(Result<cv::_InputArray*>)
	}
	
	void cv_VectorOfSize_output_array(std::vector<cv::Size>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			Ok(new cv::_OutputArray(*instance), ocvrs_return);
		} OCVRS_CATCH(Result<cv::_OutputArray*>)
	}
	
	void cv_VectorOfSize_input_output_array(std::vector<cv::Size>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			Ok(new cv::_InputOutputArray(*instance), ocvrs_return);
		} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
	}
	
}


