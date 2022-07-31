extern "C" {
	void cv_VectorOfPlatformInfo_delete(std::vector<cv::ocl::PlatformInfo>* instance) {
		delete instance;
	}

	std::vector<cv::ocl::PlatformInfo>* cv_VectorOfPlatformInfo_new() {
		return new std::vector<cv::ocl::PlatformInfo>();
	}

	size_t cv_VectorOfPlatformInfo_len(const std::vector<cv::ocl::PlatformInfo>* instance) {
		return instance->size();
	}

	bool cv_VectorOfPlatformInfo_is_empty(const std::vector<cv::ocl::PlatformInfo>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfPlatformInfo_capacity(const std::vector<cv::ocl::PlatformInfo>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfPlatformInfo_resize(std::vector<cv::ocl::PlatformInfo>* instance, size_t new_size) {
		instance->resize(new_size);
	}

	void cv_VectorOfPlatformInfo_shrink_to_fit(std::vector<cv::ocl::PlatformInfo>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfPlatformInfo_reserve(std::vector<cv::ocl::PlatformInfo>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfPlatformInfo_remove(std::vector<cv::ocl::PlatformInfo>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfPlatformInfo_swap(std::vector<cv::ocl::PlatformInfo>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfPlatformInfo_clear(std::vector<cv::ocl::PlatformInfo>* instance) {
		instance->clear();
	}

	void cv_VectorOfPlatformInfo_push(std::vector<cv::ocl::PlatformInfo>* instance, cv::ocl::PlatformInfo* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfPlatformInfo_insert(std::vector<cv::ocl::PlatformInfo>* instance, size_t index, cv::ocl::PlatformInfo* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfPlatformInfo_get(const std::vector<cv::ocl::PlatformInfo>* instance, size_t index, cv::ocl::PlatformInfo** ocvrs_return) {
		*ocvrs_return = new cv::ocl::PlatformInfo((*instance)[index]);
	}

	void cv_VectorOfPlatformInfo_set(std::vector<cv::ocl::PlatformInfo>* instance, size_t index, cv::ocl::PlatformInfo* val) {
		(*instance)[index] = *val;
	}

}


