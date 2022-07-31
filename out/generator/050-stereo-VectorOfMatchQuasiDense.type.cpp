extern "C" {
	void cv_VectorOfMatchQuasiDense_delete(std::vector<cv::stereo::MatchQuasiDense>* instance) {
		delete instance;
	}

	std::vector<cv::stereo::MatchQuasiDense>* cv_VectorOfMatchQuasiDense_new() {
		return new std::vector<cv::stereo::MatchQuasiDense>();
	}

	size_t cv_VectorOfMatchQuasiDense_len(const std::vector<cv::stereo::MatchQuasiDense>* instance) {
		return instance->size();
	}

	bool cv_VectorOfMatchQuasiDense_is_empty(const std::vector<cv::stereo::MatchQuasiDense>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfMatchQuasiDense_capacity(const std::vector<cv::stereo::MatchQuasiDense>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfMatchQuasiDense_resize(std::vector<cv::stereo::MatchQuasiDense>* instance, size_t new_size) {
		instance->resize(new_size);
	}

	void cv_VectorOfMatchQuasiDense_shrink_to_fit(std::vector<cv::stereo::MatchQuasiDense>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfMatchQuasiDense_reserve(std::vector<cv::stereo::MatchQuasiDense>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfMatchQuasiDense_remove(std::vector<cv::stereo::MatchQuasiDense>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfMatchQuasiDense_swap(std::vector<cv::stereo::MatchQuasiDense>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfMatchQuasiDense_clear(std::vector<cv::stereo::MatchQuasiDense>* instance) {
		instance->clear();
	}

	void cv_VectorOfMatchQuasiDense_push(std::vector<cv::stereo::MatchQuasiDense>* instance, cv::stereo::MatchQuasiDense* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfMatchQuasiDense_insert(std::vector<cv::stereo::MatchQuasiDense>* instance, size_t index, cv::stereo::MatchQuasiDense* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfMatchQuasiDense_get(const std::vector<cv::stereo::MatchQuasiDense>* instance, size_t index, cv::stereo::MatchQuasiDense* ocvrs_return) {
		*ocvrs_return = (*instance)[index];
	}

	void cv_VectorOfMatchQuasiDense_set(std::vector<cv::stereo::MatchQuasiDense>* instance, size_t index, cv::stereo::MatchQuasiDense* val) {
		(*instance)[index] = *val;
	}

	const cv::stereo::MatchQuasiDense* cv_VectorOfMatchQuasiDense_data(const std::vector<cv::stereo::MatchQuasiDense>* instance) {
		return instance->data();
	}
	
	cv::stereo::MatchQuasiDense* cv_VectorOfMatchQuasiDense_data_mut(std::vector<cv::stereo::MatchQuasiDense>* instance) {
		return instance->data();
	}
	
	std::vector<cv::stereo::MatchQuasiDense>* cv_VectorOfMatchQuasiDense_clone(const std::vector<cv::stereo::MatchQuasiDense>* instance) {
		return new std::vector<cv::stereo::MatchQuasiDense>(*instance);
	}
	
	std::vector<cv::stereo::MatchQuasiDense>* cv_VectorOfMatchQuasiDense_from_slice(const cv::stereo::MatchQuasiDense* data, size_t len) {
		return new std::vector<cv::stereo::MatchQuasiDense>(data, data + len);
	}
}


