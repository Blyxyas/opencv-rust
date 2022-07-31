extern "C" {
	void cv_VectorOfPose3DPtr_delete(std::vector<cv::ppf_match_3d::Pose3DPtr>* instance) {
		delete instance;
	}

	std::vector<cv::ppf_match_3d::Pose3DPtr>* cv_VectorOfPose3DPtr_new() {
		return new std::vector<cv::ppf_match_3d::Pose3DPtr>();
	}

	size_t cv_VectorOfPose3DPtr_len(const std::vector<cv::ppf_match_3d::Pose3DPtr>* instance) {
		return instance->size();
	}

	bool cv_VectorOfPose3DPtr_is_empty(const std::vector<cv::ppf_match_3d::Pose3DPtr>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfPose3DPtr_capacity(const std::vector<cv::ppf_match_3d::Pose3DPtr>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfPose3DPtr_resize(std::vector<cv::ppf_match_3d::Pose3DPtr>* instance, size_t new_size) {
		instance->resize(new_size);
	}

	void cv_VectorOfPose3DPtr_shrink_to_fit(std::vector<cv::ppf_match_3d::Pose3DPtr>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfPose3DPtr_reserve(std::vector<cv::ppf_match_3d::Pose3DPtr>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfPose3DPtr_remove(std::vector<cv::ppf_match_3d::Pose3DPtr>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfPose3DPtr_swap(std::vector<cv::ppf_match_3d::Pose3DPtr>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfPose3DPtr_clear(std::vector<cv::ppf_match_3d::Pose3DPtr>* instance) {
		instance->clear();
	}

	void cv_VectorOfPose3DPtr_push(std::vector<cv::ppf_match_3d::Pose3DPtr>* instance, cv::ppf_match_3d::Pose3DPtr* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfPose3DPtr_insert(std::vector<cv::ppf_match_3d::Pose3DPtr>* instance, size_t index, cv::ppf_match_3d::Pose3DPtr* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfPose3DPtr_get(const std::vector<cv::ppf_match_3d::Pose3DPtr>* instance, size_t index, cv::ppf_match_3d::Pose3DPtr** ocvrs_return) {
		*ocvrs_return = new cv::ppf_match_3d::Pose3DPtr((*instance)[index]);
	}

	void cv_VectorOfPose3DPtr_set(std::vector<cv::ppf_match_3d::Pose3DPtr>* instance, size_t index, cv::ppf_match_3d::Pose3DPtr* val) {
		(*instance)[index] = *val;
	}

}


