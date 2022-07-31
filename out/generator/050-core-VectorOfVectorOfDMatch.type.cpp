extern "C" {
	void cv_VectorOfVectorOfDMatch_delete(std::vector<std::vector<cv::DMatch>>* instance) {
		delete instance;
	}

	std::vector<std::vector<cv::DMatch>>* cv_VectorOfVectorOfDMatch_new() {
		return new std::vector<std::vector<cv::DMatch>>();
	}

	size_t cv_VectorOfVectorOfDMatch_len(const std::vector<std::vector<cv::DMatch>>* instance) {
		return instance->size();
	}

	bool cv_VectorOfVectorOfDMatch_is_empty(const std::vector<std::vector<cv::DMatch>>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfVectorOfDMatch_capacity(const std::vector<std::vector<cv::DMatch>>* instance) {
		return instance->capacity();
	}

	void cv_<parameter not found>_resize(std::vector<std::vector<cv::DMatch>>* instance, size_t new_size) {
		instance->resize(new_size)
	}

	void cv_VectorOfVectorOfDMatch_shrink_to_fit(std::vector<std::vector<cv::DMatch>>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfVectorOfDMatch_reserve(std::vector<std::vector<cv::DMatch>>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfVectorOfDMatch_remove(std::vector<std::vector<cv::DMatch>>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfVectorOfDMatch_swap(std::vector<std::vector<cv::DMatch>>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfVectorOfDMatch_clear(std::vector<std::vector<cv::DMatch>>* instance) {
		instance->clear();
	}

	void cv_VectorOfVectorOfDMatch_push(std::vector<std::vector<cv::DMatch>>* instance, std::vector<cv::DMatch>* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfVectorOfDMatch_insert(std::vector<std::vector<cv::DMatch>>* instance, size_t index, std::vector<cv::DMatch>* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfVectorOfDMatch_get(const std::vector<std::vector<cv::DMatch>>* instance, size_t index, std::vector<cv::DMatch>** ocvrs_return) {
		*ocvrs_return = new std::vector<cv::DMatch>((*instance)[index]);
	}

	void cv_VectorOfVectorOfDMatch_set(std::vector<std::vector<cv::DMatch>>* instance, size_t index, std::vector<cv::DMatch>* val) {
		(*instance)[index] = *val;
	}

}


