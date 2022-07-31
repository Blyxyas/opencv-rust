extern "C" {
	void cv_VectorOfPtrOfBackendWrapper_delete(std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance) {
		delete instance;
	}

	std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* cv_VectorOfPtrOfBackendWrapper_new() {
		return new std::vector<cv::Ptr<cv::dnn::BackendWrapper>>();
	}

	size_t cv_VectorOfPtrOfBackendWrapper_len(const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance) {
		return instance->size();
	}

	bool cv_VectorOfPtrOfBackendWrapper_is_empty(const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfPtrOfBackendWrapper_capacity(const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfPtrOfBackendWrapper_resize(std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance, size_t new_size) {
		instance->resize(new_size);
	}

	void cv_VectorOfPtrOfBackendWrapper_shrink_to_fit(std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfPtrOfBackendWrapper_reserve(std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfPtrOfBackendWrapper_remove(std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfPtrOfBackendWrapper_swap(std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfPtrOfBackendWrapper_clear(std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance) {
		instance->clear();
	}

	void cv_VectorOfPtrOfBackendWrapper_push(std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance, cv::Ptr<cv::dnn::BackendWrapper>* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfPtrOfBackendWrapper_insert(std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance, size_t index, cv::Ptr<cv::dnn::BackendWrapper>* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfPtrOfBackendWrapper_get(const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance, size_t index, cv::Ptr<cv::dnn::BackendWrapper>** ocvrs_return) {
		*ocvrs_return = new cv::Ptr<cv::dnn::BackendWrapper>((*instance)[index]);
	}

	void cv_VectorOfPtrOfBackendWrapper_set(std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance, size_t index, cv::Ptr<cv::dnn::BackendWrapper>* val) {
		(*instance)[index] = *val;
	}

}


