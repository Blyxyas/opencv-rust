extern "C" {
	void cv_VectorOfFacemarkAAM_Model_Texture_delete(std::vector<cv::face::FacemarkAAM::Model::Texture>* instance) {
		delete instance;
	}

	std::vector<cv::face::FacemarkAAM::Model::Texture>* cv_VectorOfFacemarkAAM_Model_Texture_new() {
		return new std::vector<cv::face::FacemarkAAM::Model::Texture>();
	}

	size_t cv_VectorOfFacemarkAAM_Model_Texture_len(const std::vector<cv::face::FacemarkAAM::Model::Texture>* instance) {
		return instance->size();
	}

	bool cv_VectorOfFacemarkAAM_Model_Texture_is_empty(const std::vector<cv::face::FacemarkAAM::Model::Texture>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfFacemarkAAM_Model_Texture_capacity(const std::vector<cv::face::FacemarkAAM::Model::Texture>* instance) {
		return instance->capacity();
	}

	void cv_<parameter not found>_resize(std::vector<cv::face::FacemarkAAM::Model::Texture>* instance, size_t new_size) {
		instance->resize(new_size)
	}

	void cv_VectorOfFacemarkAAM_Model_Texture_shrink_to_fit(std::vector<cv::face::FacemarkAAM::Model::Texture>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfFacemarkAAM_Model_Texture_reserve(std::vector<cv::face::FacemarkAAM::Model::Texture>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfFacemarkAAM_Model_Texture_remove(std::vector<cv::face::FacemarkAAM::Model::Texture>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfFacemarkAAM_Model_Texture_swap(std::vector<cv::face::FacemarkAAM::Model::Texture>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfFacemarkAAM_Model_Texture_clear(std::vector<cv::face::FacemarkAAM::Model::Texture>* instance) {
		instance->clear();
	}

	void cv_VectorOfFacemarkAAM_Model_Texture_push(std::vector<cv::face::FacemarkAAM::Model::Texture>* instance, cv::face::FacemarkAAM::Model::Texture* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfFacemarkAAM_Model_Texture_insert(std::vector<cv::face::FacemarkAAM::Model::Texture>* instance, size_t index, cv::face::FacemarkAAM::Model::Texture* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfFacemarkAAM_Model_Texture_get(const std::vector<cv::face::FacemarkAAM::Model::Texture>* instance, size_t index, cv::face::FacemarkAAM::Model::Texture** ocvrs_return) {
		*ocvrs_return = new cv::face::FacemarkAAM::Model::Texture((*instance)[index]);
	}

	void cv_VectorOfFacemarkAAM_Model_Texture_set(std::vector<cv::face::FacemarkAAM::Model::Texture>* instance, size_t index, cv::face::FacemarkAAM::Model::Texture* val) {
		(*instance)[index] = *val;
	}

}


