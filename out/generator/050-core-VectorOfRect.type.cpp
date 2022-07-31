extern "C" {
	void cv_VectorOfRect_delete(std::vector<cv::Rect>* instance) {
		delete instance;
	}

	std::vector<cv::Rect>* cv_VectorOfRect_new() {
		return new std::vector<cv::Rect>();
	}

	size_t cv_VectorOfRect_len(const std::vector<cv::Rect>* instance) {
		return instance->size();
	}

	bool cv_VectorOfRect_is_empty(const std::vector<cv::Rect>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfRect_capacity(const std::vector<cv::Rect>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfRect_resize(std::vector<cv::Rect>* instance, size_t new_size) {
		instance->resize(new_size);
	}

	void cv_VectorOfRect_shrink_to_fit(std::vector<cv::Rect>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfRect_reserve(std::vector<cv::Rect>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfRect_remove(std::vector<cv::Rect>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfRect_swap(std::vector<cv::Rect>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfRect_clear(std::vector<cv::Rect>* instance) {
		instance->clear();
	}

	void cv_VectorOfRect_push(std::vector<cv::Rect>* instance, cv::Rect* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfRect_insert(std::vector<cv::Rect>* instance, size_t index, cv::Rect* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfRect_get(const std::vector<cv::Rect>* instance, size_t index, cv::Rect* ocvrs_return) {
		*ocvrs_return = (*instance)[index];
	}

	void cv_VectorOfRect_set(std::vector<cv::Rect>* instance, size_t index, cv::Rect* val) {
		(*instance)[index] = *val;
	}

	const cv::Rect* cv_VectorOfRect_data(const std::vector<cv::Rect>* instance) {
		return instance->data();
	}
	
	cv::Rect* cv_VectorOfRect_data_mut(std::vector<cv::Rect>* instance) {
		return instance->data();
	}
	
	std::vector<cv::Rect>* cv_VectorOfRect_clone(const std::vector<cv::Rect>* instance) {
		return new std::vector<cv::Rect>(*instance);
	}
	
	std::vector<cv::Rect>* cv_VectorOfRect_from_slice(const cv::Rect* data, size_t len) {
		return new std::vector<cv::Rect>(data, data + len);
	}
	void cv_VectorOfRect_input_array(std::vector<cv::Rect>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			Ok(new cv::_InputArray(*instance), ocvrs_return);
		} OCVRS_CATCH(Result<cv::_InputArray*>)
	}
	
	void cv_VectorOfRect_output_array(std::vector<cv::Rect>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			Ok(new cv::_OutputArray(*instance), ocvrs_return);
		} OCVRS_CATCH(Result<cv::_OutputArray*>)
	}
	
	void cv_VectorOfRect_input_output_array(std::vector<cv::Rect>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			Ok(new cv::_InputOutputArray(*instance), ocvrs_return);
		} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
	}
	
}


