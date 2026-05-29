package com.dxsrc.aiteach.service.impl;

import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.dxsrc.aiteach.entity.Homework;
import com.dxsrc.aiteach.mapper.HomeworkMapper;
import com.dxsrc.aiteach.service.HomeworkFeedbackService;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
public class HomeworkFeedbackServiceImpl extends ServiceImpl<HomeworkMapper, Homework> implements HomeworkFeedbackService {

    @Override
    public List<Homework> list() {
        return super.list();
    }

    @Override
    public Homework getById(Long id) {
        return super.getById(id);
    }

    @Override
    public boolean save(Homework entity) {
        return super.save(entity);
    }

    @Override
    public boolean update(Homework entity) {
        return super.updateById(entity);
    }

    @Override
    public boolean removeById(Long id) {
        return super.removeById(id);
    }
}
