extern "C" {
	void cv_VectorOfPoint_delete(std::vector<cv::Point>* instance) {
		delete instance;
	}

	std::vector<cv::Point>* cv_VectorOfPoint_new() {
		return new std::vector<cv::Point>();
	}

	size_t cv_VectorOfPoint_len(const std::vector<cv::Point>* instance) {
		return instance->size();
	}

	bool cv_VectorOfPoint_is_empty(const std::vector<cv::Point>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfPoint_capacity(const std::vector<cv::Point>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfPoint_resize(std::vector<cv::Point>* instance, size_t new_size) {
		instance->resize(new_size);
	}

	void cv_VectorOfPoint_shrink_to_fit(std::vector<cv::Point>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfPoint_reserve(std::vector<cv::Point>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfPoint_remove(std::vector<cv::Point>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfPoint_swap(std::vector<cv::Point>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfPoint_clear(std::vector<cv::Point>* instance) {
		instance->clear();
	}

	void cv_VectorOfPoint_push(std::vector<cv::Point>* instance, cv::Point* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfPoint_insert(std::vector<cv::Point>* instance, size_t index, cv::Point* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfPoint_get(const std::vector<cv::Point>* instance, size_t index, cv::Point* ocvrs_return) {
		*ocvrs_return = (*instance)[index];
	}

	void cv_VectorOfPoint_set(std::vector<cv::Point>* instance, size_t index, cv::Point* val) {
		(*instance)[index] = *val;
	}

	const cv::Point* cv_VectorOfPoint_data(const std::vector<cv::Point>* instance) {
		return instance->data();
	}
	
	cv::Point* cv_VectorOfPoint_data_mut(std::vector<cv::Point>* instance) {
		return instance->data();
	}
	
	std::vector<cv::Point>* cv_VectorOfPoint_clone(const std::vector<cv::Point>* instance) {
		return new std::vector<cv::Point>(*instance);
	}
	
	std::vector<cv::Point>* cv_VectorOfPoint_from_slice(const cv::Point* data, size_t len) {
		return new std::vector<cv::Point>(data, data + len);
	}
	void cv_VectorOfPoint_input_array(std::vector<cv::Point>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			Ok(new cv::_InputArray(*instance), ocvrs_return);
		} OCVRS_CATCH(Result<cv::_InputArray*>)
	}
	
	void cv_VectorOfPoint_output_array(std::vector<cv::Point>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			Ok(new cv::_OutputArray(*instance), ocvrs_return);
		} OCVRS_CATCH(Result<cv::_OutputArray*>)
	}
	
	void cv_VectorOfPoint_input_output_array(std::vector<cv::Point>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			Ok(new cv::_InputOutputArray(*instance), ocvrs_return);
		} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
	}
	
}


