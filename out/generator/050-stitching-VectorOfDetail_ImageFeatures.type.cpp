extern "C" {
	void cv_VectorOfDetail_ImageFeatures_delete(std::vector<cv::detail::ImageFeatures>* instance) {
		delete instance;
	}

	std::vector<cv::detail::ImageFeatures>* cv_VectorOfDetail_ImageFeatures_new() {
		return new std::vector<cv::detail::ImageFeatures>();
	}

	size_t cv_VectorOfDetail_ImageFeatures_len(const std::vector<cv::detail::ImageFeatures>* instance) {
		return instance->size();
	}

	bool cv_VectorOfDetail_ImageFeatures_is_empty(const std::vector<cv::detail::ImageFeatures>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfDetail_ImageFeatures_capacity(const std::vector<cv::detail::ImageFeatures>* instance) {
		return instance->capacity();
	}

	void cv_<parameter not found>_resize(std::vector<cv::detail::ImageFeatures>* instance, size_t new_size) {
		instance->resize(new_size)
	}

	void cv_VectorOfDetail_ImageFeatures_shrink_to_fit(std::vector<cv::detail::ImageFeatures>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfDetail_ImageFeatures_reserve(std::vector<cv::detail::ImageFeatures>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfDetail_ImageFeatures_remove(std::vector<cv::detail::ImageFeatures>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfDetail_ImageFeatures_swap(std::vector<cv::detail::ImageFeatures>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfDetail_ImageFeatures_clear(std::vector<cv::detail::ImageFeatures>* instance) {
		instance->clear();
	}

	void cv_VectorOfDetail_ImageFeatures_push(std::vector<cv::detail::ImageFeatures>* instance, cv::detail::ImageFeatures* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfDetail_ImageFeatures_insert(std::vector<cv::detail::ImageFeatures>* instance, size_t index, cv::detail::ImageFeatures* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfDetail_ImageFeatures_get(const std::vector<cv::detail::ImageFeatures>* instance, size_t index, cv::detail::ImageFeatures** ocvrs_return) {
		*ocvrs_return = new cv::detail::ImageFeatures((*instance)[index]);
	}

	void cv_VectorOfDetail_ImageFeatures_set(std::vector<cv::detail::ImageFeatures>* instance, size_t index, cv::detail::ImageFeatures* val) {
		(*instance)[index] = *val;
	}

}


