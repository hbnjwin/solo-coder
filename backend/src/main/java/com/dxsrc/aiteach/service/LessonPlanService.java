package com.dxsrc.aiteach.service;

import com.dxsrc.aiteach.entity.Subject;
import java.util.List;

public interface LessonPlanService {

    List<Subject> list();

    Subject getById(Long id);

    boolean save(Subject entity);

    boolean update(Subject entity);

    boolean removeById(Long id);
}
