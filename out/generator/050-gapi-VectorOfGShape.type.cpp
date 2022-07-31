extern "C" {
	void cv_VectorOfGShape_delete(std::vector<cv::GShape>* instance) {
		delete instance;
	}

	std::vector<cv::GShape>* cv_VectorOfGShape_new() {
		return new std::vector<cv::GShape>();
	}

	size_t cv_VectorOfGShape_len(const std::vector<cv::GShape>* instance) {
		return instance->size();
	}

	bool cv_VectorOfGShape_is_empty(const std::vector<cv::GShape>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfGShape_capacity(const std::vector<cv::GShape>* instance) {
		return instance->capacity();
	}

	void cv_<parameter not found>_resize(std::vector<cv::GShape>* instance, size_t new_size) {
		instance->resize(new_size)
	}

	void cv_VectorOfGShape_shrink_to_fit(std::vector<cv::GShape>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfGShape_reserve(std::vector<cv::GShape>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfGShape_remove(std::vector<cv::GShape>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfGShape_swap(std::vector<cv::GShape>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfGShape_clear(std::vector<cv::GShape>* instance) {
		instance->clear();
	}

	void cv_VectorOfGShape_push(std::vector<cv::GShape>* instance, cv::GShape val) {
		instance->push_back(val);
	}

	void cv_VectorOfGShape_insert(std::vector<cv::GShape>* instance, size_t index, cv::GShape val) {
		instance->insert(instance->begin() + index, val);
	}

	void cv_VectorOfGShape_get(const std::vector<cv::GShape>* instance, size_t index, cv::GShape* ocvrs_return) {
		*ocvrs_return = (*instance)[index];
	}

	void cv_VectorOfGShape_set(std::vector<cv::GShape>* instance, size_t index, cv::GShape val) {
		(*instance)[index] = val;
	}

	const cv::GShape* cv_VectorOfGShape_data(const std::vector<cv::GShape>* instance) {
		return instance->data();
	}
	
	cv::GShape* cv_VectorOfGShape_data_mut(std::vector<cv::GShape>* instance) {
		return instance->data();
	}
	
	std::vector<cv::GShape>* cv_VectorOfGShape_clone(const std::vector<cv::GShape>* instance) {
		return new std::vector<cv::GShape>(*instance);
	}
	
	std::vector<cv::GShape>* cv_VectorOfGShape_from_slice(const cv::GShape* data, size_t len) {
		return new std::vector<cv::GShape>(data, data + len);
	}
}


