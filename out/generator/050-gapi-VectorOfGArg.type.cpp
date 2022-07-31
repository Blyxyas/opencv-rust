extern "C" {
	void cv_VectorOfGArg_delete(std::vector<cv::GArg>* instance) {
		delete instance;
	}

	std::vector<cv::GArg>* cv_VectorOfGArg_new() {
		return new std::vector<cv::GArg>();
	}

	size_t cv_VectorOfGArg_len(const std::vector<cv::GArg>* instance) {
		return instance->size();
	}

	bool cv_VectorOfGArg_is_empty(const std::vector<cv::GArg>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfGArg_capacity(const std::vector<cv::GArg>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfGArg_resize(std::vector<cv::GArg>* instance, size_t new_size) {
		instance->resize(new_size);
	}

	void cv_VectorOfGArg_shrink_to_fit(std::vector<cv::GArg>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfGArg_reserve(std::vector<cv::GArg>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfGArg_remove(std::vector<cv::GArg>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfGArg_swap(std::vector<cv::GArg>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfGArg_clear(std::vector<cv::GArg>* instance) {
		instance->clear();
	}

	void cv_VectorOfGArg_push(std::vector<cv::GArg>* instance, cv::GArg* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfGArg_insert(std::vector<cv::GArg>* instance, size_t index, cv::GArg* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfGArg_get(const std::vector<cv::GArg>* instance, size_t index, cv::GArg** ocvrs_return) {
		*ocvrs_return = new cv::GArg((*instance)[index]);
	}

	void cv_VectorOfGArg_set(std::vector<cv::GArg>* instance, size_t index, cv::GArg* val) {
		(*instance)[index] = *val;
	}

}


