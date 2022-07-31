extern "C" {
	void cv_VectorOfPtrOfLayer_delete(std::vector<cv::Ptr<cv::dnn::Layer>>* instance) {
		delete instance;
	}

	std::vector<cv::Ptr<cv::dnn::Layer>>* cv_VectorOfPtrOfLayer_new() {
		return new std::vector<cv::Ptr<cv::dnn::Layer>>();
	}

	size_t cv_VectorOfPtrOfLayer_len(const std::vector<cv::Ptr<cv::dnn::Layer>>* instance) {
		return instance->size();
	}

	bool cv_VectorOfPtrOfLayer_is_empty(const std::vector<cv::Ptr<cv::dnn::Layer>>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfPtrOfLayer_capacity(const std::vector<cv::Ptr<cv::dnn::Layer>>* instance) {
		return instance->capacity();
	}

	void cv_<parameter not found>_resize(std::vector<cv::Ptr<cv::dnn::Layer>>* instance, size_t new_size) {
		instance->resize(new_size)
	}

	void cv_VectorOfPtrOfLayer_shrink_to_fit(std::vector<cv::Ptr<cv::dnn::Layer>>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfPtrOfLayer_reserve(std::vector<cv::Ptr<cv::dnn::Layer>>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfPtrOfLayer_remove(std::vector<cv::Ptr<cv::dnn::Layer>>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfPtrOfLayer_swap(std::vector<cv::Ptr<cv::dnn::Layer>>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfPtrOfLayer_clear(std::vector<cv::Ptr<cv::dnn::Layer>>* instance) {
		instance->clear();
	}

	void cv_VectorOfPtrOfLayer_push(std::vector<cv::Ptr<cv::dnn::Layer>>* instance, cv::Ptr<cv::dnn::Layer>* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfPtrOfLayer_insert(std::vector<cv::Ptr<cv::dnn::Layer>>* instance, size_t index, cv::Ptr<cv::dnn::Layer>* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfPtrOfLayer_get(const std::vector<cv::Ptr<cv::dnn::Layer>>* instance, size_t index, cv::Ptr<cv::dnn::Layer>** ocvrs_return) {
		*ocvrs_return = new cv::Ptr<cv::dnn::Layer>((*instance)[index]);
	}

	void cv_VectorOfPtrOfLayer_set(std::vector<cv::Ptr<cv::dnn::Layer>>* instance, size_t index, cv::Ptr<cv::dnn::Layer>* val) {
		(*instance)[index] = *val;
	}

}


