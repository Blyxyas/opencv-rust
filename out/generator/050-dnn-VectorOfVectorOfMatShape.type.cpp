extern "C" {
	void cv_VectorOfVectorOfMatShape_delete(std::vector<std::vector<cv::dnn::MatShape>>* instance) {
		delete instance;
	}

	std::vector<std::vector<cv::dnn::MatShape>>* cv_VectorOfVectorOfMatShape_new() {
		return new std::vector<std::vector<cv::dnn::MatShape>>();
	}

	size_t cv_VectorOfVectorOfMatShape_len(const std::vector<std::vector<cv::dnn::MatShape>>* instance) {
		return instance->size();
	}

	bool cv_VectorOfVectorOfMatShape_is_empty(const std::vector<std::vector<cv::dnn::MatShape>>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfVectorOfMatShape_capacity(const std::vector<std::vector<cv::dnn::MatShape>>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfVectorOfMatShape_resize(std::vector<std::vector<cv::dnn::MatShape>>* instance, size_t new_size) {
		instance->resize(new_size);
	}

	void cv_VectorOfVectorOfMatShape_shrink_to_fit(std::vector<std::vector<cv::dnn::MatShape>>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfVectorOfMatShape_reserve(std::vector<std::vector<cv::dnn::MatShape>>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfVectorOfMatShape_remove(std::vector<std::vector<cv::dnn::MatShape>>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfVectorOfMatShape_swap(std::vector<std::vector<cv::dnn::MatShape>>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfVectorOfMatShape_clear(std::vector<std::vector<cv::dnn::MatShape>>* instance) {
		instance->clear();
	}

	void cv_VectorOfVectorOfMatShape_push(std::vector<std::vector<cv::dnn::MatShape>>* instance, std::vector<cv::dnn::MatShape>* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfVectorOfMatShape_insert(std::vector<std::vector<cv::dnn::MatShape>>* instance, size_t index, std::vector<cv::dnn::MatShape>* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfVectorOfMatShape_get(const std::vector<std::vector<cv::dnn::MatShape>>* instance, size_t index, std::vector<cv::dnn::MatShape>** ocvrs_return) {
		*ocvrs_return = new std::vector<cv::dnn::MatShape>((*instance)[index]);
	}

	void cv_VectorOfVectorOfMatShape_set(std::vector<std::vector<cv::dnn::MatShape>>* instance, size_t index, std::vector<cv::dnn::MatShape>* val) {
		(*instance)[index] = *val;
	}

}


