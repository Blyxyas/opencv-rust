extern "C" {
	void cv_VectorOfDTrees_Node_delete(std::vector<cv::ml::DTrees::Node>* instance) {
		delete instance;
	}

	std::vector<cv::ml::DTrees::Node>* cv_VectorOfDTrees_Node_new() {
		return new std::vector<cv::ml::DTrees::Node>();
	}

	size_t cv_VectorOfDTrees_Node_len(const std::vector<cv::ml::DTrees::Node>* instance) {
		return instance->size();
	}

	bool cv_VectorOfDTrees_Node_is_empty(const std::vector<cv::ml::DTrees::Node>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfDTrees_Node_capacity(const std::vector<cv::ml::DTrees::Node>* instance) {
		return instance->capacity();
	}

	void cv_<parameter not found>_resize(std::vector<cv::ml::DTrees::Node>* instance, size_t new_size) {
		instance->resize(new_size)
	}

	void cv_VectorOfDTrees_Node_shrink_to_fit(std::vector<cv::ml::DTrees::Node>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfDTrees_Node_reserve(std::vector<cv::ml::DTrees::Node>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfDTrees_Node_remove(std::vector<cv::ml::DTrees::Node>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfDTrees_Node_swap(std::vector<cv::ml::DTrees::Node>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfDTrees_Node_clear(std::vector<cv::ml::DTrees::Node>* instance) {
		instance->clear();
	}

	void cv_VectorOfDTrees_Node_push(std::vector<cv::ml::DTrees::Node>* instance, cv::ml::DTrees::Node* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfDTrees_Node_insert(std::vector<cv::ml::DTrees::Node>* instance, size_t index, cv::ml::DTrees::Node* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfDTrees_Node_get(const std::vector<cv::ml::DTrees::Node>* instance, size_t index, cv::ml::DTrees::Node** ocvrs_return) {
		*ocvrs_return = new cv::ml::DTrees::Node((*instance)[index]);
	}

	void cv_VectorOfDTrees_Node_set(std::vector<cv::ml::DTrees::Node>* instance, size_t index, cv::ml::DTrees::Node* val) {
		(*instance)[index] = *val;
	}

}


