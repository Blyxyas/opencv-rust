extern "C" {
	void cv_VectorOfVectorOfMat_delete(std::vector<std::vector<cv::Mat>>* instance) {
		delete instance;
	}

	std::vector<std::vector<cv::Mat>>* cv_VectorOfVectorOfMat_new() {
		return new std::vector<std::vector<cv::Mat>>();
	}

	size_t cv_VectorOfVectorOfMat_len(const std::vector<std::vector<cv::Mat>>* instance) {
		return instance->size();
	}

	bool cv_VectorOfVectorOfMat_is_empty(const std::vector<std::vector<cv::Mat>>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfVectorOfMat_capacity(const std::vector<std::vector<cv::Mat>>* instance) {
		return instance->capacity();
	}

	void cv_<parameter not found>_resize(std::vector<std::vector<cv::Mat>>* instance, size_t new_size) {
		instance->resize(new_size)
	}

	void cv_VectorOfVectorOfMat_shrink_to_fit(std::vector<std::vector<cv::Mat>>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfVectorOfMat_reserve(std::vector<std::vector<cv::Mat>>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfVectorOfMat_remove(std::vector<std::vector<cv::Mat>>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfVectorOfMat_swap(std::vector<std::vector<cv::Mat>>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfVectorOfMat_clear(std::vector<std::vector<cv::Mat>>* instance) {
		instance->clear();
	}

	void cv_VectorOfVectorOfMat_push(std::vector<std::vector<cv::Mat>>* instance, std::vector<cv::Mat>* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfVectorOfMat_insert(std::vector<std::vector<cv::Mat>>* instance, size_t index, std::vector<cv::Mat>* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfVectorOfMat_get(const std::vector<std::vector<cv::Mat>>* instance, size_t index, std::vector<cv::Mat>** ocvrs_return) {
		*ocvrs_return = new std::vector<cv::Mat>((*instance)[index]);
	}

	void cv_VectorOfVectorOfMat_set(std::vector<std::vector<cv::Mat>>* instance, size_t index, std::vector<cv::Mat>* val) {
		(*instance)[index] = *val;
	}

}


