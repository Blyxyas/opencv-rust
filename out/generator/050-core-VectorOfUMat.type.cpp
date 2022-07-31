extern "C" {
	void cv_VectorOfUMat_delete(std::vector<cv::UMat>* instance) {
		delete instance;
	}

	std::vector<cv::UMat>* cv_VectorOfUMat_new() {
		return new std::vector<cv::UMat>();
	}

	size_t cv_VectorOfUMat_len(const std::vector<cv::UMat>* instance) {
		return instance->size();
	}

	bool cv_VectorOfUMat_is_empty(const std::vector<cv::UMat>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfUMat_capacity(const std::vector<cv::UMat>* instance) {
		return instance->capacity();
	}

	void cv_<parameter not found>_resize(std::vector<cv::UMat>* instance, size_t new_size) {
		instance->resize(new_size)
	}

	void cv_VectorOfUMat_shrink_to_fit(std::vector<cv::UMat>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfUMat_reserve(std::vector<cv::UMat>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfUMat_remove(std::vector<cv::UMat>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfUMat_swap(std::vector<cv::UMat>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfUMat_clear(std::vector<cv::UMat>* instance) {
		instance->clear();
	}

	void cv_VectorOfUMat_push(std::vector<cv::UMat>* instance, cv::UMat* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfUMat_insert(std::vector<cv::UMat>* instance, size_t index, cv::UMat* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfUMat_get(const std::vector<cv::UMat>* instance, size_t index, cv::UMat** ocvrs_return) {
		*ocvrs_return = new cv::UMat((*instance)[index]);
	}

	void cv_VectorOfUMat_set(std::vector<cv::UMat>* instance, size_t index, cv::UMat* val) {
		(*instance)[index] = *val;
	}

}


