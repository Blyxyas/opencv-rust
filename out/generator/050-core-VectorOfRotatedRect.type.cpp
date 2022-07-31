extern "C" {
	void cv_VectorOfRotatedRect_delete(std::vector<cv::RotatedRect>* instance) {
		delete instance;
	}

	std::vector<cv::RotatedRect>* cv_VectorOfRotatedRect_new() {
		return new std::vector<cv::RotatedRect>();
	}

	size_t cv_VectorOfRotatedRect_len(const std::vector<cv::RotatedRect>* instance) {
		return instance->size();
	}

	bool cv_VectorOfRotatedRect_is_empty(const std::vector<cv::RotatedRect>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfRotatedRect_capacity(const std::vector<cv::RotatedRect>* instance) {
		return instance->capacity();
	}

	void cv_<parameter not found>_resize(std::vector<cv::RotatedRect>* instance, size_t new_size) {
		instance->resize(new_size)
	}

	void cv_VectorOfRotatedRect_shrink_to_fit(std::vector<cv::RotatedRect>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfRotatedRect_reserve(std::vector<cv::RotatedRect>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfRotatedRect_remove(std::vector<cv::RotatedRect>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfRotatedRect_swap(std::vector<cv::RotatedRect>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfRotatedRect_clear(std::vector<cv::RotatedRect>* instance) {
		instance->clear();
	}

	void cv_VectorOfRotatedRect_push(std::vector<cv::RotatedRect>* instance, cv::RotatedRect* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfRotatedRect_insert(std::vector<cv::RotatedRect>* instance, size_t index, cv::RotatedRect* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfRotatedRect_get(const std::vector<cv::RotatedRect>* instance, size_t index, cv::RotatedRect** ocvrs_return) {
		*ocvrs_return = new cv::RotatedRect((*instance)[index]);
	}

	void cv_VectorOfRotatedRect_set(std::vector<cv::RotatedRect>* instance, size_t index, cv::RotatedRect* val) {
		(*instance)[index] = *val;
	}

}


