extern "C" {
	void cv_VectorOfVectorOfKeyLine_delete(std::vector<std::vector<cv::line_descriptor::KeyLine>>* instance) {
		delete instance;
	}

	std::vector<std::vector<cv::line_descriptor::KeyLine>>* cv_VectorOfVectorOfKeyLine_new() {
		return new std::vector<std::vector<cv::line_descriptor::KeyLine>>();
	}

	size_t cv_VectorOfVectorOfKeyLine_len(const std::vector<std::vector<cv::line_descriptor::KeyLine>>* instance) {
		return instance->size();
	}

	bool cv_VectorOfVectorOfKeyLine_is_empty(const std::vector<std::vector<cv::line_descriptor::KeyLine>>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfVectorOfKeyLine_capacity(const std::vector<std::vector<cv::line_descriptor::KeyLine>>* instance) {
		return instance->capacity();
	}

	void cv_<parameter not found>_resize(std::vector<std::vector<cv::line_descriptor::KeyLine>>* instance, size_t new_size) {
		instance->resize(new_size)
	}

	void cv_VectorOfVectorOfKeyLine_shrink_to_fit(std::vector<std::vector<cv::line_descriptor::KeyLine>>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfVectorOfKeyLine_reserve(std::vector<std::vector<cv::line_descriptor::KeyLine>>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfVectorOfKeyLine_remove(std::vector<std::vector<cv::line_descriptor::KeyLine>>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfVectorOfKeyLine_swap(std::vector<std::vector<cv::line_descriptor::KeyLine>>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfVectorOfKeyLine_clear(std::vector<std::vector<cv::line_descriptor::KeyLine>>* instance) {
		instance->clear();
	}

	void cv_VectorOfVectorOfKeyLine_push(std::vector<std::vector<cv::line_descriptor::KeyLine>>* instance, std::vector<cv::line_descriptor::KeyLine>* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfVectorOfKeyLine_insert(std::vector<std::vector<cv::line_descriptor::KeyLine>>* instance, size_t index, std::vector<cv::line_descriptor::KeyLine>* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfVectorOfKeyLine_get(const std::vector<std::vector<cv::line_descriptor::KeyLine>>* instance, size_t index, std::vector<cv::line_descriptor::KeyLine>** ocvrs_return) {
		*ocvrs_return = new std::vector<cv::line_descriptor::KeyLine>((*instance)[index]);
	}

	void cv_VectorOfVectorOfKeyLine_set(std::vector<std::vector<cv::line_descriptor::KeyLine>>* instance, size_t index, std::vector<cv::line_descriptor::KeyLine>* val) {
		(*instance)[index] = *val;
	}

}


