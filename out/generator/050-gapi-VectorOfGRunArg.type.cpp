extern "C" {
	void cv_VectorOfGRunArg_delete(std::vector<cv::GRunArg>* instance) {
		delete instance;
	}

	std::vector<cv::GRunArg>* cv_VectorOfGRunArg_new() {
		return new std::vector<cv::GRunArg>();
	}

	size_t cv_VectorOfGRunArg_len(const std::vector<cv::GRunArg>* instance) {
		return instance->size();
	}

	bool cv_VectorOfGRunArg_is_empty(const std::vector<cv::GRunArg>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfGRunArg_capacity(const std::vector<cv::GRunArg>* instance) {
		return instance->capacity();
	}

	void cv_<parameter not found>_resize(std::vector<cv::GRunArg>* instance, size_t new_size) {
		instance->resize(new_size)
	}

	void cv_VectorOfGRunArg_shrink_to_fit(std::vector<cv::GRunArg>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfGRunArg_reserve(std::vector<cv::GRunArg>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfGRunArg_remove(std::vector<cv::GRunArg>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfGRunArg_swap(std::vector<cv::GRunArg>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfGRunArg_clear(std::vector<cv::GRunArg>* instance) {
		instance->clear();
	}

	void cv_VectorOfGRunArg_push(std::vector<cv::GRunArg>* instance, cv::GRunArg* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfGRunArg_insert(std::vector<cv::GRunArg>* instance, size_t index, cv::GRunArg* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfGRunArg_get(const std::vector<cv::GRunArg>* instance, size_t index, cv::GRunArg** ocvrs_return) {
		*ocvrs_return = new cv::GRunArg((*instance)[index]);
	}

	void cv_VectorOfGRunArg_set(std::vector<cv::GRunArg>* instance, size_t index, cv::GRunArg* val) {
		(*instance)[index] = *val;
	}

}


