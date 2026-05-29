package com.dxsrc.aiteach.service.impl;

import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.dxsrc.aiteach.entity.Classroom;
import com.dxsrc.aiteach.mapper.ClassroomMapper;
import com.dxsrc.aiteach.service.LiveQuizService;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
public class LiveQuizServiceImpl extends ServiceImpl<ClassroomMapper, Classroom> implements LiveQuizService {

    @Override
    public List<Classroom> list() {
        return super.list();
    }

    @Override
    public Classroom getById(Long id) {
        return super.getById(id);
    }

    @Override
    public boolean save(Classroom entity) {
        return super.save(entity);
    }

    @Override
    public boolean update(Classroom entity) {
        return super.updateById(entity);
    }

    @Override
    public boolean removeById(Long id) {
        return super.removeById(id);
    }
}
