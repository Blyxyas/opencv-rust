extern "C" {
	void cv_VectorOfScalar_delete(std::vector<cv::Scalar>* instance) {
		delete instance;
	}

	std::vector<cv::Scalar>* cv_VectorOfScalar_new() {
		return new std::vector<cv::Scalar>();
	}

	size_t cv_VectorOfScalar_len(const std::vector<cv::Scalar>* instance) {
		return instance->size();
	}

	bool cv_VectorOfScalar_is_empty(const std::vector<cv::Scalar>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfScalar_capacity(const std::vector<cv::Scalar>* instance) {
		return instance->capacity();
	}

	void cv_<parameter not found>_resize(std::vector<cv::Scalar>* instance, size_t new_size) {
		instance->resize(new_size)
	}

	void cv_VectorOfScalar_shrink_to_fit(std::vector<cv::Scalar>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfScalar_reserve(std::vector<cv::Scalar>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfScalar_remove(std::vector<cv::Scalar>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfScalar_swap(std::vector<cv::Scalar>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfScalar_clear(std::vector<cv::Scalar>* instance) {
		instance->clear();
	}

	void cv_VectorOfScalar_push(std::vector<cv::Scalar>* instance, cv::Scalar* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfScalar_insert(std::vector<cv::Scalar>* instance, size_t index, cv::Scalar* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfScalar_get(const std::vector<cv::Scalar>* instance, size_t index, cv::Scalar* ocvrs_return) {
		*ocvrs_return = (*instance)[index];
	}

	void cv_VectorOfScalar_set(std::vector<cv::Scalar>* instance, size_t index, cv::Scalar* val) {
		(*instance)[index] = *val;
	}

	const cv::Scalar* cv_VectorOfScalar_data(const std::vector<cv::Scalar>* instance) {
		return instance->data();
	}
	
	cv::Scalar* cv_VectorOfScalar_data_mut(std::vector<cv::Scalar>* instance) {
		return instance->data();
	}
	
	std::vector<cv::Scalar>* cv_VectorOfScalar_clone(const std::vector<cv::Scalar>* instance) {
		return new std::vector<cv::Scalar>(*instance);
	}
	
	std::vector<cv::Scalar>* cv_VectorOfScalar_from_slice(const cv::Scalar* data, size_t len) {
		return new std::vector<cv::Scalar>(data, data + len);
	}
	void cv_VectorOfScalar_input_array(std::vector<cv::Scalar>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			Ok(new cv::_InputArray(*instance), ocvrs_return);
		} OCVRS_CATCH(Result<cv::_InputArray*>)
	}
	
	void cv_VectorOfScalar_output_array(std::vector<cv::Scalar>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			Ok(new cv::_OutputArray(*instance), ocvrs_return);
		} OCVRS_CATCH(Result<cv::_OutputArray*>)
	}
	
	void cv_VectorOfScalar_input_output_array(std::vector<cv::Scalar>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			Ok(new cv::_InputOutputArray(*instance), ocvrs_return);
		} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
	}
	
}


