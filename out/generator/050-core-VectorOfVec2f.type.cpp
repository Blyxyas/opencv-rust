extern "C" {
	void cv_VectorOfVec2f_delete(std::vector<cv::Vec2f>* instance) {
		delete instance;
	}

	std::vector<cv::Vec2f>* cv_VectorOfVec2f_new() {
		return new std::vector<cv::Vec2f>();
	}

	size_t cv_VectorOfVec2f_len(const std::vector<cv::Vec2f>* instance) {
		return instance->size();
	}

	bool cv_VectorOfVec2f_is_empty(const std::vector<cv::Vec2f>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfVec2f_capacity(const std::vector<cv::Vec2f>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfVec2f_resize(std::vector<cv::Vec2f>* instance, size_t new_size) {
		instance->resize(new_size);
	}

	void cv_VectorOfVec2f_shrink_to_fit(std::vector<cv::Vec2f>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfVec2f_reserve(std::vector<cv::Vec2f>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfVec2f_remove(std::vector<cv::Vec2f>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfVec2f_swap(std::vector<cv::Vec2f>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfVec2f_clear(std::vector<cv::Vec2f>* instance) {
		instance->clear();
	}

	void cv_VectorOfVec2f_push(std::vector<cv::Vec2f>* instance, cv::Vec2f* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfVec2f_insert(std::vector<cv::Vec2f>* instance, size_t index, cv::Vec2f* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfVec2f_get(const std::vector<cv::Vec2f>* instance, size_t index, cv::Vec2f* ocvrs_return) {
		*ocvrs_return = (*instance)[index];
	}

	void cv_VectorOfVec2f_set(std::vector<cv::Vec2f>* instance, size_t index, cv::Vec2f* val) {
		(*instance)[index] = *val;
	}

	const cv::Vec2f* cv_VectorOfVec2f_data(const std::vector<cv::Vec2f>* instance) {
		return instance->data();
	}
	
	cv::Vec2f* cv_VectorOfVec2f_data_mut(std::vector<cv::Vec2f>* instance) {
		return instance->data();
	}
	
	std::vector<cv::Vec2f>* cv_VectorOfVec2f_clone(const std::vector<cv::Vec2f>* instance) {
		return new std::vector<cv::Vec2f>(*instance);
	}
	
	std::vector<cv::Vec2f>* cv_VectorOfVec2f_from_slice(const cv::Vec2f* data, size_t len) {
		return new std::vector<cv::Vec2f>(data, data + len);
	}
	void cv_VectorOfVec2f_input_array(std::vector<cv::Vec2f>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			Ok(new cv::_InputArray(*instance), ocvrs_return);
		} OCVRS_CATCH(Result<cv::_InputArray*>)
	}
	
	void cv_VectorOfVec2f_output_array(std::vector<cv::Vec2f>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			Ok(new cv::_OutputArray(*instance), ocvrs_return);
		} OCVRS_CATCH(Result<cv::_OutputArray*>)
	}
	
	void cv_VectorOfVec2f_input_output_array(std::vector<cv::Vec2f>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			Ok(new cv::_InputOutputArray(*instance), ocvrs_return);
		} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
	}
	
}


