extern "C" {
	void cv_VectorOfVectorOfRange_delete(std::vector<std::vector<cv::Range>>* instance) {
		delete instance;
	}

	std::vector<std::vector<cv::Range>>* cv_VectorOfVectorOfRange_new() {
		return new std::vector<std::vector<cv::Range>>();
	}

	size_t cv_VectorOfVectorOfRange_len(const std::vector<std::vector<cv::Range>>* instance) {
		return instance->size();
	}

	bool cv_VectorOfVectorOfRange_is_empty(const std::vector<std::vector<cv::Range>>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfVectorOfRange_capacity(const std::vector<std::vector<cv::Range>>* instance) {
		return instance->capacity();
	}

	void cv_<parameter not found>_resize(std::vector<std::vector<cv::Range>>* instance, size_t new_size) {
		instance->resize(new_size)
	}

	void cv_VectorOfVectorOfRange_shrink_to_fit(std::vector<std::vector<cv::Range>>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfVectorOfRange_reserve(std::vector<std::vector<cv::Range>>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfVectorOfRange_remove(std::vector<std::vector<cv::Range>>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfVectorOfRange_swap(std::vector<std::vector<cv::Range>>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfVectorOfRange_clear(std::vector<std::vector<cv::Range>>* instance) {
		instance->clear();
	}

	void cv_VectorOfVectorOfRange_push(std::vector<std::vector<cv::Range>>* instance, std::vector<cv::Range>* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfVectorOfRange_insert(std::vector<std::vector<cv::Range>>* instance, size_t index, std::vector<cv::Range>* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfVectorOfRange_get(const std::vector<std::vector<cv::Range>>* instance, size_t index, std::vector<cv::Range>** ocvrs_return) {
		*ocvrs_return = new std::vector<cv::Range>((*instance)[index]);
	}

	void cv_VectorOfVectorOfRange_set(std::vector<std::vector<cv::Range>>* instance, size_t index, std::vector<cv::Range>* val) {
		(*instance)[index] = *val;
	}

}


