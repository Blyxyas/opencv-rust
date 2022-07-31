extern "C" {
	void cv_VectorOfPtrOfBackendNode_delete(std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance) {
		delete instance;
	}

	std::vector<cv::Ptr<cv::dnn::BackendNode>>* cv_VectorOfPtrOfBackendNode_new() {
		return new std::vector<cv::Ptr<cv::dnn::BackendNode>>();
	}

	size_t cv_VectorOfPtrOfBackendNode_len(const std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance) {
		return instance->size();
	}

	bool cv_VectorOfPtrOfBackendNode_is_empty(const std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfPtrOfBackendNode_capacity(const std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfPtrOfBackendNode_resize(std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance, size_t new_size) {
		instance->resize(new_size);
	}

	void cv_VectorOfPtrOfBackendNode_shrink_to_fit(std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfPtrOfBackendNode_reserve(std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfPtrOfBackendNode_remove(std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfPtrOfBackendNode_swap(std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfPtrOfBackendNode_clear(std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance) {
		instance->clear();
	}

	void cv_VectorOfPtrOfBackendNode_push(std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance, cv::Ptr<cv::dnn::BackendNode>* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfPtrOfBackendNode_insert(std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance, size_t index, cv::Ptr<cv::dnn::BackendNode>* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfPtrOfBackendNode_get(const std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance, size_t index, cv::Ptr<cv::dnn::BackendNode>** ocvrs_return) {
		*ocvrs_return = new cv::Ptr<cv::dnn::BackendNode>((*instance)[index]);
	}

	void cv_VectorOfPtrOfBackendNode_set(std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance, size_t index, cv::Ptr<cv::dnn::BackendNode>* val) {
		(*instance)[index] = *val;
	}

}


