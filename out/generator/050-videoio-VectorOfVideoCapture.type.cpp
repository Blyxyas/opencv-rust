extern "C" {
	void cv_VectorOfVideoCapture_delete(std::vector<cv::VideoCapture>* instance) {
		delete instance;
	}

	std::vector<cv::VideoCapture>* cv_VectorOfVideoCapture_new() {
		return new std::vector<cv::VideoCapture>();
	}

	size_t cv_VectorOfVideoCapture_len(const std::vector<cv::VideoCapture>* instance) {
		return instance->size();
	}

	bool cv_VectorOfVideoCapture_is_empty(const std::vector<cv::VideoCapture>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfVideoCapture_capacity(const std::vector<cv::VideoCapture>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfVideoCapture_resize(std::vector<cv::VideoCapture>* instance, size_t new_size) {
		instance->resize(new_size);
	}

	void cv_VectorOfVideoCapture_shrink_to_fit(std::vector<cv::VideoCapture>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfVideoCapture_reserve(std::vector<cv::VideoCapture>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfVideoCapture_remove(std::vector<cv::VideoCapture>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfVideoCapture_swap(std::vector<cv::VideoCapture>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfVideoCapture_clear(std::vector<cv::VideoCapture>* instance) {
		instance->clear();
	}

	void cv_VectorOfVideoCapture_push(std::vector<cv::VideoCapture>* instance, cv::VideoCapture* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfVideoCapture_insert(std::vector<cv::VideoCapture>* instance, size_t index, cv::VideoCapture* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfVideoCapture_get(const std::vector<cv::VideoCapture>* instance, size_t index, cv::VideoCapture** ocvrs_return) {
		*ocvrs_return = new cv::VideoCapture((*instance)[index]);
	}

	void cv_VectorOfVideoCapture_set(std::vector<cv::VideoCapture>* instance, size_t index, cv::VideoCapture* val) {
		(*instance)[index] = *val;
	}

}


