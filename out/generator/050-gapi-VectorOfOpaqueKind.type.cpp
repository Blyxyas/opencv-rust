extern "C" {
	void cv_VectorOfOpaqueKind_delete(std::vector<cv::detail::OpaqueKind>* instance) {
		delete instance;
	}

	std::vector<cv::detail::OpaqueKind>* cv_VectorOfOpaqueKind_new() {
		return new std::vector<cv::detail::OpaqueKind>();
	}

	size_t cv_VectorOfOpaqueKind_len(const std::vector<cv::detail::OpaqueKind>* instance) {
		return instance->size();
	}

	bool cv_VectorOfOpaqueKind_is_empty(const std::vector<cv::detail::OpaqueKind>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfOpaqueKind_capacity(const std::vector<cv::detail::OpaqueKind>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfOpaqueKind_resize(std::vector<cv::detail::OpaqueKind>* instance, size_t new_size) {
		instance->resize(new_size);
	}

	void cv_VectorOfOpaqueKind_shrink_to_fit(std::vector<cv::detail::OpaqueKind>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfOpaqueKind_reserve(std::vector<cv::detail::OpaqueKind>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfOpaqueKind_remove(std::vector<cv::detail::OpaqueKind>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfOpaqueKind_swap(std::vector<cv::detail::OpaqueKind>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfOpaqueKind_clear(std::vector<cv::detail::OpaqueKind>* instance) {
		instance->clear();
	}

	void cv_VectorOfOpaqueKind_push(std::vector<cv::detail::OpaqueKind>* instance, cv::detail::OpaqueKind val) {
		instance->push_back(val);
	}

	void cv_VectorOfOpaqueKind_insert(std::vector<cv::detail::OpaqueKind>* instance, size_t index, cv::detail::OpaqueKind val) {
		instance->insert(instance->begin() + index, val);
	}

	void cv_VectorOfOpaqueKind_get(const std::vector<cv::detail::OpaqueKind>* instance, size_t index, cv::detail::OpaqueKind* ocvrs_return) {
		*ocvrs_return = (*instance)[index];
	}

	void cv_VectorOfOpaqueKind_set(std::vector<cv::detail::OpaqueKind>* instance, size_t index, cv::detail::OpaqueKind val) {
		(*instance)[index] = val;
	}

	const cv::detail::OpaqueKind* cv_VectorOfOpaqueKind_data(const std::vector<cv::detail::OpaqueKind>* instance) {
		return instance->data();
	}
	
	cv::detail::OpaqueKind* cv_VectorOfOpaqueKind_data_mut(std::vector<cv::detail::OpaqueKind>* instance) {
		return instance->data();
	}
	
	std::vector<cv::detail::OpaqueKind>* cv_VectorOfOpaqueKind_clone(const std::vector<cv::detail::OpaqueKind>* instance) {
		return new std::vector<cv::detail::OpaqueKind>(*instance);
	}
	
	std::vector<cv::detail::OpaqueKind>* cv_VectorOfOpaqueKind_from_slice(const cv::detail::OpaqueKind* data, size_t len) {
		return new std::vector<cv::detail::OpaqueKind>(data, data + len);
	}
}


