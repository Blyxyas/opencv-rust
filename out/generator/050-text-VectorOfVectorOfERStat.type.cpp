extern "C" {
	void cv_VectorOfVectorOfERStat_delete(std::vector<std::vector<cv::text::ERStat>>* instance) {
		delete instance;
	}

	std::vector<std::vector<cv::text::ERStat>>* cv_VectorOfVectorOfERStat_new() {
		return new std::vector<std::vector<cv::text::ERStat>>();
	}

	size_t cv_VectorOfVectorOfERStat_len(const std::vector<std::vector<cv::text::ERStat>>* instance) {
		return instance->size();
	}

	bool cv_VectorOfVectorOfERStat_is_empty(const std::vector<std::vector<cv::text::ERStat>>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfVectorOfERStat_capacity(const std::vector<std::vector<cv::text::ERStat>>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfVectorOfERStat_resize(std::vector<std::vector<cv::text::ERStat>>* instance, size_t new_size) {
		instance->resize(new_size);
	}

	void cv_VectorOfVectorOfERStat_shrink_to_fit(std::vector<std::vector<cv::text::ERStat>>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfVectorOfERStat_reserve(std::vector<std::vector<cv::text::ERStat>>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfVectorOfERStat_remove(std::vector<std::vector<cv::text::ERStat>>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfVectorOfERStat_swap(std::vector<std::vector<cv::text::ERStat>>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfVectorOfERStat_clear(std::vector<std::vector<cv::text::ERStat>>* instance) {
		instance->clear();
	}

	void cv_VectorOfVectorOfERStat_push(std::vector<std::vector<cv::text::ERStat>>* instance, std::vector<cv::text::ERStat>* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfVectorOfERStat_insert(std::vector<std::vector<cv::text::ERStat>>* instance, size_t index, std::vector<cv::text::ERStat>* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfVectorOfERStat_get(const std::vector<std::vector<cv::text::ERStat>>* instance, size_t index, std::vector<cv::text::ERStat>** ocvrs_return) {
		*ocvrs_return = new std::vector<cv::text::ERStat>((*instance)[index]);
	}

	void cv_VectorOfVectorOfERStat_set(std::vector<std::vector<cv::text::ERStat>>* instance, size_t index, std::vector<cv::text::ERStat>* val) {
		(*instance)[index] = *val;
	}

}


