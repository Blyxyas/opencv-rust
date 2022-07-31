extern "C" {
	void cv_VectorOfRange_delete(std::vector<cv::Range>* instance) {
		delete instance;
	}

	std::vector<cv::Range>* cv_VectorOfRange_new() {
		return new std::vector<cv::Range>();
	}

	size_t cv_VectorOfRange_len(const std::vector<cv::Range>* instance) {
		return instance->size();
	}

	bool cv_VectorOfRange_is_empty(const std::vector<cv::Range>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfRange_capacity(const std::vector<cv::Range>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfRange_resize(std::vector<cv::Range>* instance, size_t new_size) {
		instance->resize(new_size);
	}

	void cv_VectorOfRange_shrink_to_fit(std::vector<cv::Range>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfRange_reserve(std::vector<cv::Range>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfRange_remove(std::vector<cv::Range>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfRange_swap(std::vector<cv::Range>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfRange_clear(std::vector<cv::Range>* instance) {
		instance->clear();
	}

	void cv_VectorOfRange_push(std::vector<cv::Range>* instance, cv::Range* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfRange_insert(std::vector<cv::Range>* instance, size_t index, cv::Range* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfRange_get(const std::vector<cv::Range>* instance, size_t index, cv::Range** ocvrs_return) {
		*ocvrs_return = new cv::Range((*instance)[index]);
	}

	void cv_VectorOfRange_set(std::vector<cv::Range>* instance, size_t index, cv::Range* val) {
		(*instance)[index] = *val;
	}

}


