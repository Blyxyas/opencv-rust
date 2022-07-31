extern "C" {
	void cv_VectorOfVectorOfKeyPoint_delete(std::vector<std::vector<cv::KeyPoint>>* instance) {
		delete instance;
	}

	std::vector<std::vector<cv::KeyPoint>>* cv_VectorOfVectorOfKeyPoint_new() {
		return new std::vector<std::vector<cv::KeyPoint>>();
	}

	size_t cv_VectorOfVectorOfKeyPoint_len(const std::vector<std::vector<cv::KeyPoint>>* instance) {
		return instance->size();
	}

	bool cv_VectorOfVectorOfKeyPoint_is_empty(const std::vector<std::vector<cv::KeyPoint>>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfVectorOfKeyPoint_capacity(const std::vector<std::vector<cv::KeyPoint>>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfVectorOfKeyPoint_resize(std::vector<std::vector<cv::KeyPoint>>* instance, size_t new_size) {
		instance->resize(new_size);
	}

	void cv_VectorOfVectorOfKeyPoint_shrink_to_fit(std::vector<std::vector<cv::KeyPoint>>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfVectorOfKeyPoint_reserve(std::vector<std::vector<cv::KeyPoint>>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfVectorOfKeyPoint_remove(std::vector<std::vector<cv::KeyPoint>>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfVectorOfKeyPoint_swap(std::vector<std::vector<cv::KeyPoint>>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfVectorOfKeyPoint_clear(std::vector<std::vector<cv::KeyPoint>>* instance) {
		instance->clear();
	}

	void cv_VectorOfVectorOfKeyPoint_push(std::vector<std::vector<cv::KeyPoint>>* instance, std::vector<cv::KeyPoint>* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfVectorOfKeyPoint_insert(std::vector<std::vector<cv::KeyPoint>>* instance, size_t index, std::vector<cv::KeyPoint>* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfVectorOfKeyPoint_get(const std::vector<std::vector<cv::KeyPoint>>* instance, size_t index, std::vector<cv::KeyPoint>** ocvrs_return) {
		*ocvrs_return = new std::vector<cv::KeyPoint>((*instance)[index]);
	}

	void cv_VectorOfVectorOfKeyPoint_set(std::vector<std::vector<cv::KeyPoint>>* instance, size_t index, std::vector<cv::KeyPoint>* val) {
		(*instance)[index] = *val;
	}

}


