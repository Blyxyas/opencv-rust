extern "C" {
	void cv_VectorOfDetectionROI_delete(std::vector<cv::DetectionROI>* instance) {
		delete instance;
	}

	std::vector<cv::DetectionROI>* cv_VectorOfDetectionROI_new() {
		return new std::vector<cv::DetectionROI>();
	}

	size_t cv_VectorOfDetectionROI_len(const std::vector<cv::DetectionROI>* instance) {
		return instance->size();
	}

	bool cv_VectorOfDetectionROI_is_empty(const std::vector<cv::DetectionROI>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfDetectionROI_capacity(const std::vector<cv::DetectionROI>* instance) {
		return instance->capacity();
	}

	void cv_<parameter not found>_resize(std::vector<cv::DetectionROI>* instance, size_t new_size) {
		instance->resize(new_size)
	}

	void cv_VectorOfDetectionROI_shrink_to_fit(std::vector<cv::DetectionROI>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfDetectionROI_reserve(std::vector<cv::DetectionROI>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfDetectionROI_remove(std::vector<cv::DetectionROI>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfDetectionROI_swap(std::vector<cv::DetectionROI>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfDetectionROI_clear(std::vector<cv::DetectionROI>* instance) {
		instance->clear();
	}

	void cv_VectorOfDetectionROI_push(std::vector<cv::DetectionROI>* instance, cv::DetectionROI* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfDetectionROI_insert(std::vector<cv::DetectionROI>* instance, size_t index, cv::DetectionROI* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfDetectionROI_get(const std::vector<cv::DetectionROI>* instance, size_t index, cv::DetectionROI** ocvrs_return) {
		*ocvrs_return = new cv::DetectionROI((*instance)[index]);
	}

	void cv_VectorOfDetectionROI_set(std::vector<cv::DetectionROI>* instance, size_t index, cv::DetectionROI* val) {
		(*instance)[index] = *val;
	}

}


