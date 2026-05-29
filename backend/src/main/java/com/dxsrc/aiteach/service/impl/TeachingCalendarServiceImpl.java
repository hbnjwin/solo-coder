package com.dxsrc.aiteach.service.impl;

import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.dxsrc.aiteach.entity.CalendarEvent;
import com.dxsrc.aiteach.mapper.CalendarEventMapper;
import com.dxsrc.aiteach.service.TeachingCalendarService;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
public class TeachingCalendarServiceImpl extends ServiceImpl<CalendarEventMapper, CalendarEvent> implements TeachingCalendarService {

    @Override
    public List<CalendarEvent> list() {
        return super.list();
    }

    @Override
    public CalendarEvent getById(Long id) {
        return super.getById(id);
    }

    @Override
    public boolean save(CalendarEvent entity) {
        return super.save(entity);
    }

    @Override
    public boolean update(CalendarEvent entity) {
        return super.updateById(entity);
    }

    @Override
    public boolean removeById(Long id) {
        return super.removeById(id);
    }
}
