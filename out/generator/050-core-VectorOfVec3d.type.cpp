extern "C" {
	void cv_VectorOfVec3d_delete(std::vector<cv::Vec3d>* instance) {
		delete instance;
	}

	std::vector<cv::Vec3d>* cv_VectorOfVec3d_new() {
		return new std::vector<cv::Vec3d>();
	}

	size_t cv_VectorOfVec3d_len(const std::vector<cv::Vec3d>* instance) {
		return instance->size();
	}

	bool cv_VectorOfVec3d_is_empty(const std::vector<cv::Vec3d>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfVec3d_capacity(const std::vector<cv::Vec3d>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfVec3d_resize(std::vector<cv::Vec3d>* instance, size_t new_size) {
		instance->resize(new_size);
	}

	void cv_VectorOfVec3d_shrink_to_fit(std::vector<cv::Vec3d>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfVec3d_reserve(std::vector<cv::Vec3d>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfVec3d_remove(std::vector<cv::Vec3d>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfVec3d_swap(std::vector<cv::Vec3d>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfVec3d_clear(std::vector<cv::Vec3d>* instance) {
		instance->clear();
	}

	void cv_VectorOfVec3d_push(std::vector<cv::Vec3d>* instance, cv::Vec3d* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfVec3d_insert(std::vector<cv::Vec3d>* instance, size_t index, cv::Vec3d* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfVec3d_get(const std::vector<cv::Vec3d>* instance, size_t index, cv::Vec3d* ocvrs_return) {
		*ocvrs_return = (*instance)[index];
	}

	void cv_VectorOfVec3d_set(std::vector<cv::Vec3d>* instance, size_t index, cv::Vec3d* val) {
		(*instance)[index] = *val;
	}

	const cv::Vec3d* cv_VectorOfVec3d_data(const std::vector<cv::Vec3d>* instance) {
		return instance->data();
	}
	
	cv::Vec3d* cv_VectorOfVec3d_data_mut(std::vector<cv::Vec3d>* instance) {
		return instance->data();
	}
	
	std::vector<cv::Vec3d>* cv_VectorOfVec3d_clone(const std::vector<cv::Vec3d>* instance) {
		return new std::vector<cv::Vec3d>(*instance);
	}
	
	std::vector<cv::Vec3d>* cv_VectorOfVec3d_from_slice(const cv::Vec3d* data, size_t len) {
		return new std::vector<cv::Vec3d>(data, data + len);
	}
	void cv_VectorOfVec3d_input_array(std::vector<cv::Vec3d>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			Ok(new cv::_InputArray(*instance), ocvrs_return);
		} OCVRS_CATCH(Result<cv::_InputArray*>)
	}
	
	void cv_VectorOfVec3d_output_array(std::vector<cv::Vec3d>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			Ok(new cv::_OutputArray(*instance), ocvrs_return);
		} OCVRS_CATCH(Result<cv::_OutputArray*>)
	}
	
	void cv_VectorOfVec3d_input_output_array(std::vector<cv::Vec3d>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			Ok(new cv::_InputOutputArray(*instance), ocvrs_return);
		} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
	}
	
}


