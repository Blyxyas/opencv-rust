extern "C" {
	void cv_VectorOfLinemod_Feature_delete(std::vector<cv::linemod::Feature>* instance) {
		delete instance;
	}

	std::vector<cv::linemod::Feature>* cv_VectorOfLinemod_Feature_new() {
		return new std::vector<cv::linemod::Feature>();
	}

	size_t cv_VectorOfLinemod_Feature_len(const std::vector<cv::linemod::Feature>* instance) {
		return instance->size();
	}

	bool cv_VectorOfLinemod_Feature_is_empty(const std::vector<cv::linemod::Feature>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfLinemod_Feature_capacity(const std::vector<cv::linemod::Feature>* instance) {
		return instance->capacity();
	}

	void cv_<parameter not found>_resize(std::vector<cv::linemod::Feature>* instance, size_t new_size) {
		instance->resize(new_size)
	}

	void cv_VectorOfLinemod_Feature_shrink_to_fit(std::vector<cv::linemod::Feature>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfLinemod_Feature_reserve(std::vector<cv::linemod::Feature>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfLinemod_Feature_remove(std::vector<cv::linemod::Feature>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfLinemod_Feature_swap(std::vector<cv::linemod::Feature>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfLinemod_Feature_clear(std::vector<cv::linemod::Feature>* instance) {
		instance->clear();
	}

	void cv_VectorOfLinemod_Feature_push(std::vector<cv::linemod::Feature>* instance, cv::linemod::Feature* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfLinemod_Feature_insert(std::vector<cv::linemod::Feature>* instance, size_t index, cv::linemod::Feature* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfLinemod_Feature_get(const std::vector<cv::linemod::Feature>* instance, size_t index, cv::linemod::Feature* ocvrs_return) {
		*ocvrs_return = (*instance)[index];
	}

	void cv_VectorOfLinemod_Feature_set(std::vector<cv::linemod::Feature>* instance, size_t index, cv::linemod::Feature* val) {
		(*instance)[index] = *val;
	}

	const cv::linemod::Feature* cv_VectorOfLinemod_Feature_data(const std::vector<cv::linemod::Feature>* instance) {
		return instance->data();
	}
	
	cv::linemod::Feature* cv_VectorOfLinemod_Feature_data_mut(std::vector<cv::linemod::Feature>* instance) {
		return instance->data();
	}
	
	std::vector<cv::linemod::Feature>* cv_VectorOfLinemod_Feature_clone(const std::vector<cv::linemod::Feature>* instance) {
		return new std::vector<cv::linemod::Feature>(*instance);
	}
	
	std::vector<cv::linemod::Feature>* cv_VectorOfLinemod_Feature_from_slice(const cv::linemod::Feature* data, size_t len) {
		return new std::vector<cv::linemod::Feature>(data, data + len);
	}
}


