extern "C" {
	void cv_VectorOfDetail_MatchesInfo_delete(std::vector<cv::detail::MatchesInfo>* instance) {
		delete instance;
	}

	std::vector<cv::detail::MatchesInfo>* cv_VectorOfDetail_MatchesInfo_new() {
		return new std::vector<cv::detail::MatchesInfo>();
	}

	size_t cv_VectorOfDetail_MatchesInfo_len(const std::vector<cv::detail::MatchesInfo>* instance) {
		return instance->size();
	}

	bool cv_VectorOfDetail_MatchesInfo_is_empty(const std::vector<cv::detail::MatchesInfo>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfDetail_MatchesInfo_capacity(const std::vector<cv::detail::MatchesInfo>* instance) {
		return instance->capacity();
	}

	void cv_<parameter not found>_resize(std::vector<cv::detail::MatchesInfo>* instance, size_t new_size) {
		instance->resize(new_size)
	}

	void cv_VectorOfDetail_MatchesInfo_shrink_to_fit(std::vector<cv::detail::MatchesInfo>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfDetail_MatchesInfo_reserve(std::vector<cv::detail::MatchesInfo>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfDetail_MatchesInfo_remove(std::vector<cv::detail::MatchesInfo>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfDetail_MatchesInfo_swap(std::vector<cv::detail::MatchesInfo>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfDetail_MatchesInfo_clear(std::vector<cv::detail::MatchesInfo>* instance) {
		instance->clear();
	}

	void cv_VectorOfDetail_MatchesInfo_push(std::vector<cv::detail::MatchesInfo>* instance, cv::detail::MatchesInfo* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfDetail_MatchesInfo_insert(std::vector<cv::detail::MatchesInfo>* instance, size_t index, cv::detail::MatchesInfo* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfDetail_MatchesInfo_get(const std::vector<cv::detail::MatchesInfo>* instance, size_t index, cv::detail::MatchesInfo** ocvrs_return) {
		*ocvrs_return = new cv::detail::MatchesInfo((*instance)[index]);
	}

	void cv_VectorOfDetail_MatchesInfo_set(std::vector<cv::detail::MatchesInfo>* instance, size_t index, cv::detail::MatchesInfo* val) {
		(*instance)[index] = *val;
	}

}


