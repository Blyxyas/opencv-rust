extern "C" {
	void cv_VectorOfFacemarkAAM_Config_delete(std::vector<cv::face::FacemarkAAM::Config>* instance) {
		delete instance;
	}

	std::vector<cv::face::FacemarkAAM::Config>* cv_VectorOfFacemarkAAM_Config_new() {
		return new std::vector<cv::face::FacemarkAAM::Config>();
	}

	size_t cv_VectorOfFacemarkAAM_Config_len(const std::vector<cv::face::FacemarkAAM::Config>* instance) {
		return instance->size();
	}

	bool cv_VectorOfFacemarkAAM_Config_is_empty(const std::vector<cv::face::FacemarkAAM::Config>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfFacemarkAAM_Config_capacity(const std::vector<cv::face::FacemarkAAM::Config>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfFacemarkAAM_Config_resize(std::vector<cv::face::FacemarkAAM::Config>* instance, size_t new_size) {
		instance->resize(new_size);
	}

	void cv_VectorOfFacemarkAAM_Config_shrink_to_fit(std::vector<cv::face::FacemarkAAM::Config>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfFacemarkAAM_Config_reserve(std::vector<cv::face::FacemarkAAM::Config>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfFacemarkAAM_Config_remove(std::vector<cv::face::FacemarkAAM::Config>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfFacemarkAAM_Config_swap(std::vector<cv::face::FacemarkAAM::Config>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfFacemarkAAM_Config_clear(std::vector<cv::face::FacemarkAAM::Config>* instance) {
		instance->clear();
	}

	void cv_VectorOfFacemarkAAM_Config_push(std::vector<cv::face::FacemarkAAM::Config>* instance, cv::face::FacemarkAAM::Config* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfFacemarkAAM_Config_insert(std::vector<cv::face::FacemarkAAM::Config>* instance, size_t index, cv::face::FacemarkAAM::Config* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfFacemarkAAM_Config_get(const std::vector<cv::face::FacemarkAAM::Config>* instance, size_t index, cv::face::FacemarkAAM::Config** ocvrs_return) {
		*ocvrs_return = new cv::face::FacemarkAAM::Config((*instance)[index]);
	}

	void cv_VectorOfFacemarkAAM_Config_set(std::vector<cv::face::FacemarkAAM::Config>* instance, size_t index, cv::face::FacemarkAAM::Config* val) {
		(*instance)[index] = *val;
	}

}


