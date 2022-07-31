extern "C" {
	void cv_VectorOfDTrees_Split_delete(std::vector<cv::ml::DTrees::Split>* instance) {
		delete instance;
	}

	std::vector<cv::ml::DTrees::Split>* cv_VectorOfDTrees_Split_new() {
		return new std::vector<cv::ml::DTrees::Split>();
	}

	size_t cv_VectorOfDTrees_Split_len(const std::vector<cv::ml::DTrees::Split>* instance) {
		return instance->size();
	}

	bool cv_VectorOfDTrees_Split_is_empty(const std::vector<cv::ml::DTrees::Split>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfDTrees_Split_capacity(const std::vector<cv::ml::DTrees::Split>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfDTrees_Split_resize(std::vector<cv::ml::DTrees::Split>* instance, size_t new_size) {
		instance->resize(new_size);
	}

	void cv_VectorOfDTrees_Split_shrink_to_fit(std::vector<cv::ml::DTrees::Split>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfDTrees_Split_reserve(std::vector<cv::ml::DTrees::Split>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfDTrees_Split_remove(std::vector<cv::ml::DTrees::Split>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfDTrees_Split_swap(std::vector<cv::ml::DTrees::Split>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfDTrees_Split_clear(std::vector<cv::ml::DTrees::Split>* instance) {
		instance->clear();
	}

	void cv_VectorOfDTrees_Split_push(std::vector<cv::ml::DTrees::Split>* instance, cv::ml::DTrees::Split* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfDTrees_Split_insert(std::vector<cv::ml::DTrees::Split>* instance, size_t index, cv::ml::DTrees::Split* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfDTrees_Split_get(const std::vector<cv::ml::DTrees::Split>* instance, size_t index, cv::ml::DTrees::Split** ocvrs_return) {
		*ocvrs_return = new cv::ml::DTrees::Split((*instance)[index]);
	}

	void cv_VectorOfDTrees_Split_set(std::vector<cv::ml::DTrees::Split>* instance, size_t index, cv::ml::DTrees::Split* val) {
		(*instance)[index] = *val;
	}

}


