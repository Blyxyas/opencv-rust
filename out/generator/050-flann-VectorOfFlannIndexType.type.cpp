extern "C" {
	void cv_VectorOfFlannIndexType_delete(std::vector<cv::flann::FlannIndexType>* instance) {
		delete instance;
	}

	std::vector<cv::flann::FlannIndexType>* cv_VectorOfFlannIndexType_new() {
		return new std::vector<cv::flann::FlannIndexType>();
	}

	size_t cv_VectorOfFlannIndexType_len(const std::vector<cv::flann::FlannIndexType>* instance) {
		return instance->size();
	}

	bool cv_VectorOfFlannIndexType_is_empty(const std::vector<cv::flann::FlannIndexType>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfFlannIndexType_capacity(const std::vector<cv::flann::FlannIndexType>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfFlannIndexType_resize(std::vector<cv::flann::FlannIndexType>* instance, size_t new_size) {
		instance->resize(new_size);
	}

	void cv_VectorOfFlannIndexType_shrink_to_fit(std::vector<cv::flann::FlannIndexType>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfFlannIndexType_reserve(std::vector<cv::flann::FlannIndexType>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfFlannIndexType_remove(std::vector<cv::flann::FlannIndexType>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfFlannIndexType_swap(std::vector<cv::flann::FlannIndexType>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfFlannIndexType_clear(std::vector<cv::flann::FlannIndexType>* instance) {
		instance->clear();
	}

	void cv_VectorOfFlannIndexType_push(std::vector<cv::flann::FlannIndexType>* instance, cv::flann::FlannIndexType val) {
		instance->push_back(val);
	}

	void cv_VectorOfFlannIndexType_insert(std::vector<cv::flann::FlannIndexType>* instance, size_t index, cv::flann::FlannIndexType val) {
		instance->insert(instance->begin() + index, val);
	}

	void cv_VectorOfFlannIndexType_get(const std::vector<cv::flann::FlannIndexType>* instance, size_t index, cv::flann::FlannIndexType* ocvrs_return) {
		*ocvrs_return = (*instance)[index];
	}

	void cv_VectorOfFlannIndexType_set(std::vector<cv::flann::FlannIndexType>* instance, size_t index, cv::flann::FlannIndexType val) {
		(*instance)[index] = val;
	}

	const cv::flann::FlannIndexType* cv_VectorOfFlannIndexType_data(const std::vector<cv::flann::FlannIndexType>* instance) {
		return instance->data();
	}
	
	cv::flann::FlannIndexType* cv_VectorOfFlannIndexType_data_mut(std::vector<cv::flann::FlannIndexType>* instance) {
		return instance->data();
	}
	
	std::vector<cv::flann::FlannIndexType>* cv_VectorOfFlannIndexType_clone(const std::vector<cv::flann::FlannIndexType>* instance) {
		return new std::vector<cv::flann::FlannIndexType>(*instance);
	}
	
	std::vector<cv::flann::FlannIndexType>* cv_VectorOfFlannIndexType_from_slice(const cv::flann::FlannIndexType* data, size_t len) {
		return new std::vector<cv::flann::FlannIndexType>(data, data + len);
	}
}


