extern "C" {
	void cv_VectorOfVectorOfPoint3i_delete(std::vector<std::vector<cv::Point3i>>* instance) {
		delete instance;
	}

	std::vector<std::vector<cv::Point3i>>* cv_VectorOfVectorOfPoint3i_new() {
		return new std::vector<std::vector<cv::Point3i>>();
	}

	size_t cv_VectorOfVectorOfPoint3i_len(const std::vector<std::vector<cv::Point3i>>* instance) {
		return instance->size();
	}

	bool cv_VectorOfVectorOfPoint3i_is_empty(const std::vector<std::vector<cv::Point3i>>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfVectorOfPoint3i_capacity(const std::vector<std::vector<cv::Point3i>>* instance) {
		return instance->capacity();
	}

	void cv_<parameter not found>_resize(std::vector<std::vector<cv::Point3i>>* instance, size_t new_size) {
		instance->resize(new_size)
	}

	void cv_VectorOfVectorOfPoint3i_shrink_to_fit(std::vector<std::vector<cv::Point3i>>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfVectorOfPoint3i_reserve(std::vector<std::vector<cv::Point3i>>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfVectorOfPoint3i_remove(std::vector<std::vector<cv::Point3i>>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfVectorOfPoint3i_swap(std::vector<std::vector<cv::Point3i>>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfVectorOfPoint3i_clear(std::vector<std::vector<cv::Point3i>>* instance) {
		instance->clear();
	}

	void cv_VectorOfVectorOfPoint3i_push(std::vector<std::vector<cv::Point3i>>* instance, std::vector<cv::Point3i>* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfVectorOfPoint3i_insert(std::vector<std::vector<cv::Point3i>>* instance, size_t index, std::vector<cv::Point3i>* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfVectorOfPoint3i_get(const std::vector<std::vector<cv::Point3i>>* instance, size_t index, std::vector<cv::Point3i>** ocvrs_return) {
		*ocvrs_return = new std::vector<cv::Point3i>((*instance)[index]);
	}

	void cv_VectorOfVectorOfPoint3i_set(std::vector<std::vector<cv::Point3i>>* instance, size_t index, std::vector<cv::Point3i>* val) {
		(*instance)[index] = *val;
	}

	void cv_VectorOfVectorOfPoint3i_input_array(std::vector<std::vector<cv::Point3i>>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			Ok(new cv::_InputArray(*instance), ocvrs_return);
		} OCVRS_CATCH(Result<cv::_InputArray*>)
	}
	
	void cv_VectorOfVectorOfPoint3i_output_array(std::vector<std::vector<cv::Point3i>>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			Ok(new cv::_OutputArray(*instance), ocvrs_return);
		} OCVRS_CATCH(Result<cv::_OutputArray*>)
	}
	
	void cv_VectorOfVectorOfPoint3i_input_output_array(std::vector<std::vector<cv::Point3i>>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			Ok(new cv::_InputOutputArray(*instance), ocvrs_return);
		} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
	}
	
}


