extern "C" {
	void cv_VectorOfDetectionBasedTracker_ExtObject_delete(std::vector<cv::DetectionBasedTracker::ExtObject>* instance) {
		delete instance;
	}

	std::vector<cv::DetectionBasedTracker::ExtObject>* cv_VectorOfDetectionBasedTracker_ExtObject_new() {
		return new std::vector<cv::DetectionBasedTracker::ExtObject>();
	}

	size_t cv_VectorOfDetectionBasedTracker_ExtObject_len(const std::vector<cv::DetectionBasedTracker::ExtObject>* instance) {
		return instance->size();
	}

	bool cv_VectorOfDetectionBasedTracker_ExtObject_is_empty(const std::vector<cv::DetectionBasedTracker::ExtObject>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfDetectionBasedTracker_ExtObject_capacity(const std::vector<cv::DetectionBasedTracker::ExtObject>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfDetectionBasedTracker_ExtObject_resize(std::vector<cv::DetectionBasedTracker::ExtObject>* instance, size_t new_size) {
		instance->resize(new_size);
	}

	void cv_VectorOfDetectionBasedTracker_ExtObject_shrink_to_fit(std::vector<cv::DetectionBasedTracker::ExtObject>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfDetectionBasedTracker_ExtObject_reserve(std::vector<cv::DetectionBasedTracker::ExtObject>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfDetectionBasedTracker_ExtObject_remove(std::vector<cv::DetectionBasedTracker::ExtObject>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfDetectionBasedTracker_ExtObject_swap(std::vector<cv::DetectionBasedTracker::ExtObject>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfDetectionBasedTracker_ExtObject_clear(std::vector<cv::DetectionBasedTracker::ExtObject>* instance) {
		instance->clear();
	}

	void cv_VectorOfDetectionBasedTracker_ExtObject_push(std::vector<cv::DetectionBasedTracker::ExtObject>* instance, cv::DetectionBasedTracker::ExtObject* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfDetectionBasedTracker_ExtObject_insert(std::vector<cv::DetectionBasedTracker::ExtObject>* instance, size_t index, cv::DetectionBasedTracker::ExtObject* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfDetectionBasedTracker_ExtObject_get(const std::vector<cv::DetectionBasedTracker::ExtObject>* instance, size_t index, cv::DetectionBasedTracker::ExtObject** ocvrs_return) {
		*ocvrs_return = new cv::DetectionBasedTracker::ExtObject((*instance)[index]);
	}

	void cv_VectorOfDetectionBasedTracker_ExtObject_set(std::vector<cv::DetectionBasedTracker::ExtObject>* instance, size_t index, cv::DetectionBasedTracker::ExtObject* val) {
		(*instance)[index] = *val;
	}

}


