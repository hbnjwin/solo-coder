package com.dxsrc.aiteach.service.impl;

import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.dxsrc.aiteach.entity.Timetable;
import com.dxsrc.aiteach.mapper.TimetableMapper;
import com.dxsrc.aiteach.service.TimetableService;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
public class TimetableServiceImpl extends ServiceImpl<TimetableMapper, Timetable> implements TimetableService {

    @Override
    public List<Timetable> list() {
        return super.list();
    }

    @Override
    public Timetable getById(Long id) {
        return super.getById(id);
    }

    @Override
    public boolean save(Timetable entity) {
        return super.save(entity);
    }

    @Override
    public boolean update(Timetable entity) {
        return super.updateById(entity);
    }

    @Override
    public boolean removeById(Long id) {
        return super.removeById(id);
    }
}
