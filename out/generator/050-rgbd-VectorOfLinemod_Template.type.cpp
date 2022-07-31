extern "C" {
	void cv_VectorOfLinemod_Template_delete(std::vector<cv::linemod::Template>* instance) {
		delete instance;
	}

	std::vector<cv::linemod::Template>* cv_VectorOfLinemod_Template_new() {
		return new std::vector<cv::linemod::Template>();
	}

	size_t cv_VectorOfLinemod_Template_len(const std::vector<cv::linemod::Template>* instance) {
		return instance->size();
	}

	bool cv_VectorOfLinemod_Template_is_empty(const std::vector<cv::linemod::Template>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfLinemod_Template_capacity(const std::vector<cv::linemod::Template>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfLinemod_Template_resize(std::vector<cv::linemod::Template>* instance, size_t new_size) {
		instance->resize(new_size);
	}

	void cv_VectorOfLinemod_Template_shrink_to_fit(std::vector<cv::linemod::Template>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfLinemod_Template_reserve(std::vector<cv::linemod::Template>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfLinemod_Template_remove(std::vector<cv::linemod::Template>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfLinemod_Template_swap(std::vector<cv::linemod::Template>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfLinemod_Template_clear(std::vector<cv::linemod::Template>* instance) {
		instance->clear();
	}

	void cv_VectorOfLinemod_Template_push(std::vector<cv::linemod::Template>* instance, cv::linemod::Template* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfLinemod_Template_insert(std::vector<cv::linemod::Template>* instance, size_t index, cv::linemod::Template* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfLinemod_Template_get(const std::vector<cv::linemod::Template>* instance, size_t index, cv::linemod::Template** ocvrs_return) {
		*ocvrs_return = new cv::linemod::Template((*instance)[index]);
	}

	void cv_VectorOfLinemod_Template_set(std::vector<cv::linemod::Template>* instance, size_t index, cv::linemod::Template* val) {
		(*instance)[index] = *val;
	}

}


