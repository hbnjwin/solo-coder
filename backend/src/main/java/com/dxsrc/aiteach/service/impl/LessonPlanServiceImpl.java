package com.dxsrc.aiteach.service.impl;

import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.dxsrc.aiteach.entity.Subject;
import com.dxsrc.aiteach.mapper.SubjectMapper;
import com.dxsrc.aiteach.service.LessonPlanService;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
public class LessonPlanServiceImpl extends ServiceImpl<SubjectMapper, Subject> implements LessonPlanService {

    @Override
    public List<Subject> list() {
        return super.list();
    }

    @Override
    public Subject getById(Long id) {
        return super.getById(id);
    }

    @Override
    public boolean save(Subject entity) {
        return super.save(entity);
    }

    @Override
    public boolean update(Subject entity) {
        return super.updateById(entity);
    }

    @Override
    public boolean removeById(Long id) {
        return super.removeById(id);
    }
}
