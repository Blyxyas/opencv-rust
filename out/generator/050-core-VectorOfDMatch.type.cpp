extern "C" {
	void cv_VectorOfDMatch_delete(std::vector<cv::DMatch>* instance) {
		delete instance;
	}

	std::vector<cv::DMatch>* cv_VectorOfDMatch_new() {
		return new std::vector<cv::DMatch>();
	}

	size_t cv_VectorOfDMatch_len(const std::vector<cv::DMatch>* instance) {
		return instance->size();
	}

	bool cv_VectorOfDMatch_is_empty(const std::vector<cv::DMatch>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfDMatch_capacity(const std::vector<cv::DMatch>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfDMatch_resize(std::vector<cv::DMatch>* instance, size_t new_size) {
		instance->resize(new_size);
	}

	void cv_VectorOfDMatch_shrink_to_fit(std::vector<cv::DMatch>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfDMatch_reserve(std::vector<cv::DMatch>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfDMatch_remove(std::vector<cv::DMatch>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfDMatch_swap(std::vector<cv::DMatch>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfDMatch_clear(std::vector<cv::DMatch>* instance) {
		instance->clear();
	}

	void cv_VectorOfDMatch_push(std::vector<cv::DMatch>* instance, cv::DMatch* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfDMatch_insert(std::vector<cv::DMatch>* instance, size_t index, cv::DMatch* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfDMatch_get(const std::vector<cv::DMatch>* instance, size_t index, cv::DMatch* ocvrs_return) {
		*ocvrs_return = (*instance)[index];
	}

	void cv_VectorOfDMatch_set(std::vector<cv::DMatch>* instance, size_t index, cv::DMatch* val) {
		(*instance)[index] = *val;
	}

	const cv::DMatch* cv_VectorOfDMatch_data(const std::vector<cv::DMatch>* instance) {
		return instance->data();
	}
	
	cv::DMatch* cv_VectorOfDMatch_data_mut(std::vector<cv::DMatch>* instance) {
		return instance->data();
	}
	
	std::vector<cv::DMatch>* cv_VectorOfDMatch_clone(const std::vector<cv::DMatch>* instance) {
		return new std::vector<cv::DMatch>(*instance);
	}
	
	std::vector<cv::DMatch>* cv_VectorOfDMatch_from_slice(const cv::DMatch* data, size_t len) {
		return new std::vector<cv::DMatch>(data, data + len);
	}
}


