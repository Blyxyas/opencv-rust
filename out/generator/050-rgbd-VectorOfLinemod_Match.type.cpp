extern "C" {
	void cv_VectorOfLinemod_Match_delete(std::vector<cv::linemod::Match>* instance) {
		delete instance;
	}

	std::vector<cv::linemod::Match>* cv_VectorOfLinemod_Match_new() {
		return new std::vector<cv::linemod::Match>();
	}

	size_t cv_VectorOfLinemod_Match_len(const std::vector<cv::linemod::Match>* instance) {
		return instance->size();
	}

	bool cv_VectorOfLinemod_Match_is_empty(const std::vector<cv::linemod::Match>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfLinemod_Match_capacity(const std::vector<cv::linemod::Match>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfLinemod_Match_resize(std::vector<cv::linemod::Match>* instance, size_t new_size) {
		instance->resize(new_size);
	}

	void cv_VectorOfLinemod_Match_shrink_to_fit(std::vector<cv::linemod::Match>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfLinemod_Match_reserve(std::vector<cv::linemod::Match>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfLinemod_Match_remove(std::vector<cv::linemod::Match>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfLinemod_Match_swap(std::vector<cv::linemod::Match>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfLinemod_Match_clear(std::vector<cv::linemod::Match>* instance) {
		instance->clear();
	}

	void cv_VectorOfLinemod_Match_push(std::vector<cv::linemod::Match>* instance, cv::linemod::Match* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfLinemod_Match_insert(std::vector<cv::linemod::Match>* instance, size_t index, cv::linemod::Match* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfLinemod_Match_get(const std::vector<cv::linemod::Match>* instance, size_t index, cv::linemod::Match** ocvrs_return) {
		*ocvrs_return = new cv::linemod::Match((*instance)[index]);
	}

	void cv_VectorOfLinemod_Match_set(std::vector<cv::linemod::Match>* instance, size_t index, cv::linemod::Match* val) {
		(*instance)[index] = *val;
	}

}


