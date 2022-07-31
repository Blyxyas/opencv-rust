extern "C" {
	void cv_VectorOfDetail_CameraParams_delete(std::vector<cv::detail::CameraParams>* instance) {
		delete instance;
	}

	std::vector<cv::detail::CameraParams>* cv_VectorOfDetail_CameraParams_new() {
		return new std::vector<cv::detail::CameraParams>();
	}

	size_t cv_VectorOfDetail_CameraParams_len(const std::vector<cv::detail::CameraParams>* instance) {
		return instance->size();
	}

	bool cv_VectorOfDetail_CameraParams_is_empty(const std::vector<cv::detail::CameraParams>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfDetail_CameraParams_capacity(const std::vector<cv::detail::CameraParams>* instance) {
		return instance->capacity();
	}

	void cv_<parameter not found>_resize(std::vector<cv::detail::CameraParams>* instance, size_t new_size) {
		instance->resize(new_size)
	}

	void cv_VectorOfDetail_CameraParams_shrink_to_fit(std::vector<cv::detail::CameraParams>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfDetail_CameraParams_reserve(std::vector<cv::detail::CameraParams>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfDetail_CameraParams_remove(std::vector<cv::detail::CameraParams>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfDetail_CameraParams_swap(std::vector<cv::detail::CameraParams>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfDetail_CameraParams_clear(std::vector<cv::detail::CameraParams>* instance) {
		instance->clear();
	}

	void cv_VectorOfDetail_CameraParams_push(std::vector<cv::detail::CameraParams>* instance, cv::detail::CameraParams* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfDetail_CameraParams_insert(std::vector<cv::detail::CameraParams>* instance, size_t index, cv::detail::CameraParams* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfDetail_CameraParams_get(const std::vector<cv::detail::CameraParams>* instance, size_t index, cv::detail::CameraParams** ocvrs_return) {
		*ocvrs_return = new cv::detail::CameraParams((*instance)[index]);
	}

	void cv_VectorOfDetail_CameraParams_set(std::vector<cv::detail::CameraParams>* instance, size_t index, cv::detail::CameraParams* val) {
		(*instance)[index] = *val;
	}

}


