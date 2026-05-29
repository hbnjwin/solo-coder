package com.dxsrc.aiteach.service;

import com.dxsrc.aiteach.entity.Homework;
import java.util.List;

public interface HomeworkFeedbackService {

    List<Homework> list();

    Homework getById(Long id);

    boolean save(Homework entity);

    boolean update(Homework entity);

    boolean removeById(Long id);
}
