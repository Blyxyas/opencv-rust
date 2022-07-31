extern "C" {
	void cv_VectorOfERStat_delete(std::vector<cv::text::ERStat>* instance) {
		delete instance;
	}

	std::vector<cv::text::ERStat>* cv_VectorOfERStat_new() {
		return new std::vector<cv::text::ERStat>();
	}

	size_t cv_VectorOfERStat_len(const std::vector<cv::text::ERStat>* instance) {
		return instance->size();
	}

	bool cv_VectorOfERStat_is_empty(const std::vector<cv::text::ERStat>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfERStat_capacity(const std::vector<cv::text::ERStat>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfERStat_resize(std::vector<cv::text::ERStat>* instance, size_t new_size) {
		instance->resize(new_size);
	}

	void cv_VectorOfERStat_shrink_to_fit(std::vector<cv::text::ERStat>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfERStat_reserve(std::vector<cv::text::ERStat>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfERStat_remove(std::vector<cv::text::ERStat>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfERStat_swap(std::vector<cv::text::ERStat>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfERStat_clear(std::vector<cv::text::ERStat>* instance) {
		instance->clear();
	}

	void cv_VectorOfERStat_push(std::vector<cv::text::ERStat>* instance, cv::text::ERStat* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfERStat_insert(std::vector<cv::text::ERStat>* instance, size_t index, cv::text::ERStat* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfERStat_get(const std::vector<cv::text::ERStat>* instance, size_t index, cv::text::ERStat** ocvrs_return) {
		*ocvrs_return = new cv::text::ERStat((*instance)[index]);
	}

	void cv_VectorOfERStat_set(std::vector<cv::text::ERStat>* instance, size_t index, cv::text::ERStat* val) {
		(*instance)[index] = *val;
	}

}


