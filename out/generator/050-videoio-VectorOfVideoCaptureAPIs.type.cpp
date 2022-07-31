extern "C" {
	void cv_VectorOfVideoCaptureAPIs_delete(std::vector<cv::VideoCaptureAPIs>* instance) {
		delete instance;
	}

	std::vector<cv::VideoCaptureAPIs>* cv_VectorOfVideoCaptureAPIs_new() {
		return new std::vector<cv::VideoCaptureAPIs>();
	}

	size_t cv_VectorOfVideoCaptureAPIs_len(const std::vector<cv::VideoCaptureAPIs>* instance) {
		return instance->size();
	}

	bool cv_VectorOfVideoCaptureAPIs_is_empty(const std::vector<cv::VideoCaptureAPIs>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfVideoCaptureAPIs_capacity(const std::vector<cv::VideoCaptureAPIs>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfVideoCaptureAPIs_resize(std::vector<cv::VideoCaptureAPIs>* instance, size_t new_size) {
		instance->resize(new_size);
	}

	void cv_VectorOfVideoCaptureAPIs_shrink_to_fit(std::vector<cv::VideoCaptureAPIs>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfVideoCaptureAPIs_reserve(std::vector<cv::VideoCaptureAPIs>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfVideoCaptureAPIs_remove(std::vector<cv::VideoCaptureAPIs>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfVideoCaptureAPIs_swap(std::vector<cv::VideoCaptureAPIs>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfVideoCaptureAPIs_clear(std::vector<cv::VideoCaptureAPIs>* instance) {
		instance->clear();
	}

	void cv_VectorOfVideoCaptureAPIs_push(std::vector<cv::VideoCaptureAPIs>* instance, cv::VideoCaptureAPIs val) {
		instance->push_back(val);
	}

	void cv_VectorOfVideoCaptureAPIs_insert(std::vector<cv::VideoCaptureAPIs>* instance, size_t index, cv::VideoCaptureAPIs val) {
		instance->insert(instance->begin() + index, val);
	}

	void cv_VectorOfVideoCaptureAPIs_get(const std::vector<cv::VideoCaptureAPIs>* instance, size_t index, cv::VideoCaptureAPIs* ocvrs_return) {
		*ocvrs_return = (*instance)[index];
	}

	void cv_VectorOfVideoCaptureAPIs_set(std::vector<cv::VideoCaptureAPIs>* instance, size_t index, cv::VideoCaptureAPIs val) {
		(*instance)[index] = val;
	}

	const cv::VideoCaptureAPIs* cv_VectorOfVideoCaptureAPIs_data(const std::vector<cv::VideoCaptureAPIs>* instance) {
		return instance->data();
	}
	
	cv::VideoCaptureAPIs* cv_VectorOfVideoCaptureAPIs_data_mut(std::vector<cv::VideoCaptureAPIs>* instance) {
		return instance->data();
	}
	
	std::vector<cv::VideoCaptureAPIs>* cv_VectorOfVideoCaptureAPIs_clone(const std::vector<cv::VideoCaptureAPIs>* instance) {
		return new std::vector<cv::VideoCaptureAPIs>(*instance);
	}
	
	std::vector<cv::VideoCaptureAPIs>* cv_VectorOfVideoCaptureAPIs_from_slice(const cv::VideoCaptureAPIs* data, size_t len) {
		return new std::vector<cv::VideoCaptureAPIs>(data, data + len);
	}
}


