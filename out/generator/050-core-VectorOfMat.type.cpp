extern "C" {
	void cv_VectorOfMat_delete(std::vector<cv::Mat>* instance) {
		delete instance;
	}

	std::vector<cv::Mat>* cv_VectorOfMat_new() {
		return new std::vector<cv::Mat>();
	}

	size_t cv_VectorOfMat_len(const std::vector<cv::Mat>* instance) {
		return instance->size();
	}

	bool cv_VectorOfMat_is_empty(const std::vector<cv::Mat>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfMat_capacity(const std::vector<cv::Mat>* instance) {
		return instance->capacity();
	}

	void cv_<parameter not found>_resize(std::vector<cv::Mat>* instance, size_t new_size) {
		instance->resize(new_size)
	}

	void cv_VectorOfMat_shrink_to_fit(std::vector<cv::Mat>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfMat_reserve(std::vector<cv::Mat>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfMat_remove(std::vector<cv::Mat>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfMat_swap(std::vector<cv::Mat>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfMat_clear(std::vector<cv::Mat>* instance) {
		instance->clear();
	}

	void cv_VectorOfMat_push(std::vector<cv::Mat>* instance, cv::Mat* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfMat_insert(std::vector<cv::Mat>* instance, size_t index, cv::Mat* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfMat_get(const std::vector<cv::Mat>* instance, size_t index, cv::Mat** ocvrs_return) {
		*ocvrs_return = new cv::Mat((*instance)[index]);
	}

	void cv_VectorOfMat_set(std::vector<cv::Mat>* instance, size_t index, cv::Mat* val) {
		(*instance)[index] = *val;
	}

}


