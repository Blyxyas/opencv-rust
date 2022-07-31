extern "C" {
	void cv_VectorOfString_delete(std::vector<cv::String>* instance) {
		delete instance;
	}

	std::vector<cv::String>* cv_VectorOfString_new() {
		return new std::vector<cv::String>();
	}

	size_t cv_VectorOfString_len(const std::vector<cv::String>* instance) {
		return instance->size();
	}

	bool cv_VectorOfString_is_empty(const std::vector<cv::String>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfString_capacity(const std::vector<cv::String>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfString_resize(std::vector<cv::String>* instance, size_t new_size) {
		instance->resize(new_size);
	}

	void cv_VectorOfString_shrink_to_fit(std::vector<cv::String>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfString_reserve(std::vector<cv::String>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfString_remove(std::vector<cv::String>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfString_swap(std::vector<cv::String>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfString_clear(std::vector<cv::String>* instance) {
		instance->clear();
	}

	void cv_VectorOfString_push(std::vector<cv::String>* instance, char* val) {
		instance->push_back(std::string(val));
	}

	void cv_VectorOfString_insert(std::vector<cv::String>* instance, size_t index, char* val) {
		instance->insert(instance->begin() + index, std::string(val));
	}

	void cv_VectorOfString_get(const std::vector<cv::String>* instance, size_t index, void** ocvrs_return) {
		*ocvrs_return = ocvrs_create_string((*instance)[index].c_str());
	}

	void cv_VectorOfString_set(std::vector<cv::String>* instance, size_t index, char* val) {
		(*instance)[index] = std::string(val);
	}

}


